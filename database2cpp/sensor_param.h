pragma once

#include <array>
#include <bitset>
#include <string>
#include <vector>
#include <nlohmann/json.hpp>
#include "simcommon/error_trace_info.h"

namespace pqxx
{
class row;
}

namespace simcloud::utils
{

class SensorParamRow
{

    int64_t id_ = 0;
    int64_t sensor_list_id_ = 0;
    int64_t project_id_ = 0;
    int64_t group_id_ = 0;
    std::string car_imei_;
    std::string name_;
    std::string description_;
    std::string sensor_info_;
    bool input_enabled_ = 0;
    bool output_enabled_ = 0;
    bool downstream_enabled_ = 0;
    double input_range_min_ = 0.0;
    double input_range_max_ = 0.0;
    int32_t input_interval_ = 0;
    std::string input_param_;
    std::string input_strategy_;
    bool output_auto_mode_ = 0;
    int32_t output_heartbeat_interval_ = 0;
    int32_t output_interval_ = 0;
    std::string output_param_;
    std::string output_strategy_;
    std::string downstream_info_;
    std::string create_time_;

    enum
    {
        index_id,
        index_sensor_list_id,
        index_project_id,
        index_group_id,
        index_car_imei,
        index_name,
        index_description,
        index_sensor_info,
        index_input_enabled,
        index_output_enabled,
        index_downstream_enabled,
        index_input_range_min,
        index_input_range_max,
        index_input_interval,
        index_input_param,
        index_input_strategy,
        index_output_auto_mode,
        index_output_heartbeat_interval,
        index_output_interval,
        index_output_param,
        index_output_strategy,
        index_downstream_info,
        index_create_time,
    };
    std::bitset<index_total_count> bit_;

public:
    SensorParamRow() = default;
    SensorParamRow(const SensorParamRow&) = default;
    SensorParamRow(SensorParamRow&&) noexcept = default;
    SensorParamRow& operator=(const SensorParamRow&) = default;
    SensorParamRow& operator=(SensorParamRow&&) noexcept = default;
    ~SensorParamRow() = default;

    struct Column
    {
        static constexpr std::string_view id = "id"
        static constexpr std::string_view sensor_list_id = "sensor_list_id"
        static constexpr std::string_view project_id = "project_id"
        static constexpr std::string_view group_id = "group_id"
        static constexpr std::string_view car_imei = "car_imei"
        static constexpr std::string_view name = "name"
        static constexpr std::string_view description = "description"
        static constexpr std::string_view sensor_info = "sensor_info"
        static constexpr std::string_view input_enabled = "input_enabled"
        static constexpr std::string_view output_enabled = "output_enabled"
        static constexpr std::string_view downstream_enabled = "downstream_enabled"
        static constexpr std::string_view input_range_min = "input_range_min"
        static constexpr std::string_view input_range_max = "input_range_max"
        static constexpr std::string_view input_interval = "input_interval"
        static constexpr std::string_view input_param = "input_param"
        static constexpr std::string_view input_strategy = "input_strategy"
        static constexpr std::string_view output_auto_mode = "output_auto_mode"
        static constexpr std::string_view output_heartbeat_interval = "output_heartbeat_interval"
        static constexpr std::string_view output_interval = "output_interval"
        static constexpr std::string_view output_param = "output_param"
        static constexpr std::string_view output_strategy = "output_strategy"
        static constexpr std::string_view downstream_info = "downstream_info"
        static constexpr std::string_view create_time = "create_time"
        static constexpr std::array<std::string_view, index_total_count> placeholders = {
            "$1",
            "$2",
            "$3",
            "$4",
            "$5",
            "$6",
            "$7",
            "$8",
            "$9",
            "$10",
            "$11",
            "$12",
            "$13",
            "$14",
            "$15",
            "$16",
            "$17",
            "$18",
            "$19",
            "$20",
            "$21",
            "$22",
            "$23",
        };
        static constexpr std::array<std::string_view, index_total_count> columns = {
            id,
            sensor_list_id,
            project_id,
            group_id,
            car_imei,
            name,
            description,
            sensor_info,
            input_enabled,
            output_enabled,
            downstream_enabled,
            input_range_min,
            input_range_max,
            input_interval,
            input_param,
            input_strategy,
            output_auto_mode,
            output_heartbeat_interval,
            output_interval,
            output_param,
            output_strategy,
            downstream_info,
            create_time,
        };
        static constexpr int total_count = index_total_count;
    };
    static constexpr std::string_view table_name = "sensor_param";

