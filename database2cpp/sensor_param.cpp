#include "sensor_param.h"

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
    if (has_sensor_list_id()) set_sensor_list_id(row["sensor_list_id"].as<int64_t>());
    if (has_project_id()) set_project_id(row["project_id"].as<int64_t>());
    if (has_group_id()) set_group_id(row["group_id"].as<int64_t>());
    if (has_car_imei()) set_car_imei(row["car_imei"].as<std::string>());
    if (has_name()) set_name(row["name"].as<std::string>());
    if (has_description()) set_description(row["description"].as<std::string>());
    if (has_sensor_info()) set_sensor_info(row["sensor_info"].as<std::string>());
    if (has_input_enabled()) set_input_enabled(row["input_enabled"].as<bool>());
    if (has_output_enabled()) set_output_enabled(row["output_enabled"].as<bool>());
    if (has_downstream_enabled()) set_downstream_enabled(row["downstream_enabled"].as<bool>());
    if (has_input_range_min()) set_input_range_min(row["input_range_min"].as<double>());
    if (has_input_range_max()) set_input_range_max(row["input_range_max"].as<double>());
    if (has_input_interval()) set_input_interval(row["input_interval"].as<int32_t>());
    if (has_input_param()) set_input_param(row["input_param"].as<std::string>());
    if (has_input_strategy()) set_input_strategy(row["input_strategy"].as<std::string>());
    if (has_output_auto_mode()) set_output_auto_mode(row["output_auto_mode"].as<bool>());
    if (has_output_heartbeat_interval()) set_output_heartbeat_interval(row["output_heartbeat_interval"].as<int32_t>());
    if (has_output_interval()) set_output_interval(row["output_interval"].as<int32_t>());
    if (has_output_param()) set_output_param(row["output_param"].as<std::string>());
    if (has_output_strategy()) set_output_strategy(row["output_strategy"].as<std::string>());
    if (has_downstream_info()) set_downstream_info(row["downstream_info"].as<std::string>());
    if (has_create_time()) set_create_time(row["create_time"].as<std::string>());
}

void SensorParamRow::FromJson(const nlohmann::json& root)
{
    if (root.contains("id") && root["id"].is_number_integer())
        set_id(root["id"].get<int64_t>());
    if (root.contains("sensor_list_id") && root["sensor_list_id"].is_number_integer())
        set_sensor_list_id(root["sensor_list_id"].get<int64_t>());
    if (root.contains("project_id") && root["project_id"].is_number_integer())
        set_project_id(root["project_id"].get<int64_t>());
    if (root.contains("group_id") && root["group_id"].is_number_integer())
        set_group_id(root["group_id"].get<int64_t>());
    if (root.contains("car_imei") && root["car_imei"].is_string())
        set_car_imei(root["car_imei"].get<std::string>());
    if (root.contains("name") && root["name"].is_string())
        set_name(root["name"].get<std::string>());
    if (root.contains("description") && root["description"].is_string())
        set_description(root["description"].get<std::string>());
    if (root.contains("sensor_info") && root["sensor_info"].is_string())
        set_sensor_info(root["sensor_info"].get<std::string>());
    if (root.contains("input_enabled") && root["input_enabled"].is_boolean())
        set_input_enabled(root["input_enabled"].get<bool>());
    if (root.contains("output_enabled") && root["output_enabled"].is_boolean())
        set_output_enabled(root["output_enabled"].get<bool>());
    if (root.contains("downstream_enabled") && root["downstream_enabled"].is_boolean())
        set_downstream_enabled(root["downstream_enabled"].get<bool>());
    if (root.contains("input_range_min") && root["input_range_min"].is_number_float())
        set_input_range_min(root["input_range_min"].get<double>());
    if (root.contains("input_range_max") && root["input_range_max"].is_number_float())
        set_input_range_max(root["input_range_max"].get<double>());
    if (root.contains("input_interval") && root["input_interval"].is_number_integer())
        set_input_interval(root["input_interval"].get<int32_t>());
    if (root.contains("input_param") && root["input_param"].is_string())
        set_input_param(root["input_param"].get<std::string>());
    if (root.contains("input_strategy") && root["input_strategy"].is_string())
        set_input_strategy(root["input_strategy"].get<std::string>());
    if (root.contains("output_auto_mode") && root["output_auto_mode"].is_boolean())
        set_output_auto_mode(root["output_auto_mode"].get<bool>());
    if (root.contains("output_heartbeat_interval") && root["output_heartbeat_interval"].is_number_integer())
        set_output_heartbeat_interval(root["output_heartbeat_interval"].get<int32_t>());
    if (root.contains("output_interval") && root["output_interval"].is_number_integer())
        set_output_interval(root["output_interval"].get<int32_t>());
    if (root.contains("output_param") && root["output_param"].is_string())
        set_output_param(root["output_param"].get<std::string>());
    if (root.contains("output_strategy") && root["output_strategy"].is_string())
        set_output_strategy(root["output_strategy"].get<std::string>());
    if (root.contains("downstream_info") && root["downstream_info"].is_string())
        set_downstream_info(root["downstream_info"].get<std::string>());
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
        else if (name == sensor_list_id) set_sensor_list_id(strtoll(value.c_str(), nullptr, 10));
        else if (name == project_id) set_project_id(strtoll(value.c_str(), nullptr, 10));
        else if (name == group_id) set_group_id(strtoll(value.c_str(), nullptr, 10));
        else if (name == car_imei) set_car_imei(value);
        else if (name == name) set_name(value);
        else if (name == description) set_description(value);
        else if (name == sensor_info) set_sensor_info(value);
        else if (name == input_enabled) set_input_enabled(value == "true" || value == "1" || value == "on");
        else if (name == output_enabled) set_output_enabled(value == "true" || value == "1" || value == "on");
        else if (name == downstream_enabled) set_downstream_enabled(value == "true" || value == "1" || value == "on");
        else if (name == input_range_min) set_input_range_min(strtod(value.c_str(), nullptr));
        else if (name == input_range_max) set_input_range_max(strtod(value.c_str(), nullptr));
        else if (name == input_interval) set_input_interval(static_cast<int32_t>(strtoll(value.c_str(), nullptr, 10)));
        else if (name == input_param) set_input_param(value);
        else if (name == input_strategy) set_input_strategy(value);
        else if (name == output_auto_mode) set_output_auto_mode(value == "true" || value == "1" || value == "on");
        else if (name == output_heartbeat_interval) set_output_heartbeat_interval(static_cast<int32_t>(strtoll(value.c_str(), nullptr, 10)));
        else if (name == output_interval) set_output_interval(static_cast<int32_t>(strtoll(value.c_str(), nullptr, 10)));
        else if (name == output_param) set_output_param(value);
        else if (name == output_strategy) set_output_strategy(value);
        else if (name == downstream_info) set_downstream_info(value);
        else if (name == create_time) set_create_time(value);
    }
}

SensorParamRow& SensorParamRow::SetValidColumns()
{
    bit_.set();
    return *this;
}

SensorParamRow& SensorParamRow::SetValidColumns(const std::initializer_list<int>& valid_columns)
{
    for (const int index : valid_columns)
    {
        bit_.set(index);
    }
    return *this;
}

SensorParamRow& SensorParamRow::SetInvalidColumns()
{
    bit_.reset();
    return *this;
}

