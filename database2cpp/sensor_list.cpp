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

