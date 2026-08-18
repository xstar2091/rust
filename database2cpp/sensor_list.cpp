#include "sensor_list.h"

#include <fmt/format.h>
#include <pqxx/row>

namespace simcloud::utils
{

void SensorParamRow::FromDatabaseRow(const pqxx::row& row)
{
    if (bit_.none())
    {
        bit_.set();
    }
    if (has_id()) set_id(row["id"].as<int64_t>());
    if (has_version_major()) set_version_major(row["version_major"].as<int32_t>());
    if (has_version_minor()) set_version_minor(row["version_minor"].as<int32_t>());
    if (has_name()) set_name(row["name"].as<std::string>());
    if (has_description()) set_description(row["description"].as<std::string>());
    if (has_create_time()) set_create_time(row["create_time"].as<std::string>());
}

void SensorListRow::FromJson(const nlohmann::json& root)
{
    if (root.contains("id") && root["id"].is_number_integer())
        set_id(root["id"].get<int64_t>());
    if (root.contains("version_major") && root["version_major"].is_number_integer())
        set_version_major(root["version_major"].get<int32_t>());
    if (root.contains("version_minor") && root["version_minor"].is_number_integer())
        set_version_minor(root["version_minor"].get<int32_t>());
    if (root.contains("name") && root["name"].is_string())
        set_name(root["name"].get<std::string>());
    if (root.contains("description") && root["description"].is_string())
        set_description(root["description"].get<std::string>());
    if (root.contains("create_time") && root["create_time"].is_string())
        set_create_time(root["create_time"].get<std::string>());

    if (!root.contains("param") || !root["param"].is_array())
    {
        return;
    }
    for (const auto& param : root["param"])
    {
        if (!param.is_object() ||
            !param.contains("name") || !param["name"].is_string() ||
            !param.contains("value") || !param["value"].is_string())
        {
            continue;
        }
        const auto& name = param["name"].get_ref<const std::string&>();
        const auto& value = param["value"].get_ref<const std::string&>();
        if (name == id) set_id(strtoll(value.c_str(), nullptr, 10));
        else if (name == version_major) set_version_major(static_cast<int32_t>(strtoll(value.c_str(), nullptr, 10)));
        else if (name == version_minor) set_version_minor(static_cast<int32_t>(strtoll(value.c_str(), nullptr, 10)));
        else if (name == name) set_name(value);
        else if (name == description) set_description(value);
        else if (name == create_time) set_create_time(value);
    }
}

SensorListRow& SensorListRow::SetValidColumns()
{
    bit_.set();
    return *this;
}

SensorListRow& SensorListRow::SetValidColumns(const std::initializer_list<int>& valid_columns)
{
    for (const int index : valid_columns)
    {
        bit_.set(index);
    }
    return *this;
}

SensorListRow& SensorListRow::SetInvalidColumns()
{
    bit_.reset();
    return *this;
}

std::string SensorListRow::String(const int index) const noexcept
{
    if (index == index_id) return fmt::format("{}", id_);
    if (index == index_version_major) return fmt::format("{}", version_major_);
    if (index == index_version_minor) return fmt::format("{}", version_minor_);
    if (index == index_name) return name_;
    if (index == index_description) return description_;
    if (index == index_create_time) return create_time_;
}

nlohmann::json SensorListRow::ToJson() const
{
    nlohmann::json root = nlohmann::json::object();
    if (has_id()) root["id"] = id_;
    if (has_version_major()) root["version_major"] = version_major_;
    if (has_version_minor()) root["version_minor"] = version_minor_;
    if (has_name()) root["name"] = name_;
    if (has_description()) root["description"] = description_;
    if (has_create_time()) root["create_time"] = create_time_;

    root["param"] = nlohmann::json::array();
    auto& param = root["param"];
    if (has_version_major())
    {
        param.push_back({
            {"name", "version_major"},
            {"value", fmt::format("{}", version_major_)},
            {"desc", ""},
            {"range", []},
            {"type", "double"},
            {"unit", ""},
        });
    }
    if (has_version_minor())
    {
        param.push_back({
            {"name", "version_minor"},
            {"value", fmt::format("{}", version_minor_)},
            {"desc", ""},
            {"range", []},
            {"type", "double"},
            {"unit", ""},
        });
    }
    if (has_name())
    {
        param.push_back({
            {"name", "name"},
            {"value", name_},
            {"desc", ""},
            {"range", []},
            {"type", "string"},
            {"unit", ""},
        });
    }
    if (has_description())
    {
        param.push_back({
            {"name", "description"},
            {"value", description_},
            {"desc", ""},
            {"range", []},
            {"type", "string"},
            {"unit", ""},
        });
    }
    if (has_create_time())
    {
        param.push_back({
            {"name", "create_time"},
            {"value", create_time_},
            {"desc", ""},
            {"range", []},
            {"type", "string"},
            {"unit", ""},
        });
    }
}

