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
    bool input_enabled_ = false;
    bool output_enabled_ = false;
    bool downstream_enabled_ = false;
    double input_range_min_ = 0.0;
    double input_range_max_ = 0.0;
    int32_t input_interval_ = 0;
    std::string input_param_;
    std::string input_strategy_;
    bool output_auto_mode_ = false;
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

    [[nodiscard]] int64_t id() const noexcept { return id_; }
    [[nodiscard]] int64_t sensor_list_id() const noexcept { return sensor_list_id_; }
    [[nodiscard]] int64_t project_id() const noexcept { return project_id_; }
    [[nodiscard]] int64_t group_id() const noexcept { return group_id_; }
    [[nodiscard]] const std::string& car_imei() const noexcept { return car_imei_; }
    [[nodiscard]] const std::string& name() const noexcept { return name_; }
    [[nodiscard]] const std::string& description() const noexcept { return description_; }
    [[nodiscard]] const std::string& sensor_info() const noexcept { return sensor_info_; }
    [[nodiscard]] bool input_enabled() const noexcept { return input_enabled_; }
    [[nodiscard]] bool output_enabled() const noexcept { return output_enabled_; }
    [[nodiscard]] bool downstream_enabled() const noexcept { return downstream_enabled_; }
    [[nodiscard]] double input_range_min() const noexcept { return input_range_min_; }
    [[nodiscard]] double input_range_max() const noexcept { return input_range_max_; }
    [[nodiscard]] int32_t input_interval() const noexcept { return input_interval_; }
    [[nodiscard]] const std::string& input_param() const noexcept { return input_param_; }
    [[nodiscard]] const std::string& input_strategy() const noexcept { return input_strategy_; }
    [[nodiscard]] bool output_auto_mode() const noexcept { return output_auto_mode_; }
    [[nodiscard]] int32_t output_heartbeat_interval() const noexcept { return output_heartbeat_interval_; }
    [[nodiscard]] int32_t output_interval() const noexcept { return output_interval_; }
    [[nodiscard]] const std::string& output_param() const noexcept { return output_param_; }
    [[nodiscard]] const std::string& output_strategy() const noexcept { return output_strategy_; }
    [[nodiscard]] const std::string& downstream_info() const noexcept { return downstream_info_; }
    [[nodiscard]] const std::string& create_time() const noexcept { return create_time_; }

    [[nodiscard]] bool has(const int index) const noexcept { return bit_.test(index); }
    [[nodiscard]] bool has_none() const noexcept { return bit_.none(); }
    [[nodiscard]] bool has_all() const noexcept { return bit_.all(); }
    [[nodiscard]] bool has_any() const noexcept { return bit_.any(); }
    [[nodiscard]] bool has_id() const noexcept { return bit_.test(index_id); }
    [[nodiscard]] bool has_sensor_list_id() const noexcept { return bit_.test(index_sensor_list_id); }
    [[nodiscard]] bool has_project_id() const noexcept { return bit_.test(index_project_id); }
    [[nodiscard]] bool has_group_id() const noexcept { return bit_.test(index_group_id); }
    [[nodiscard]] bool has_car_imei() const noexcept { return bit_.test(index_car_imei); }
    [[nodiscard]] bool has_name() const noexcept { return bit_.test(index_name); }
    [[nodiscard]] bool has_description() const noexcept { return bit_.test(index_description); }
    [[nodiscard]] bool has_sensor_info() const noexcept { return bit_.test(index_sensor_info); }
    [[nodiscard]] bool has_input_enabled() const noexcept { return bit_.test(index_input_enabled); }
    [[nodiscard]] bool has_output_enabled() const noexcept { return bit_.test(index_output_enabled); }
    [[nodiscard]] bool has_downstream_enabled() const noexcept { return bit_.test(index_downstream_enabled); }
    [[nodiscard]] bool has_input_range_min() const noexcept { return bit_.test(index_input_range_min); }
    [[nodiscard]] bool has_input_range_max() const noexcept { return bit_.test(index_input_range_max); }
    [[nodiscard]] bool has_input_interval() const noexcept { return bit_.test(index_input_interval); }
    [[nodiscard]] bool has_input_param() const noexcept { return bit_.test(index_input_param); }
    [[nodiscard]] bool has_input_strategy() const noexcept { return bit_.test(index_input_strategy); }
    [[nodiscard]] bool has_output_auto_mode() const noexcept { return bit_.test(index_output_auto_mode); }
    [[nodiscard]] bool has_output_heartbeat_interval() const noexcept { return bit_.test(index_output_heartbeat_interval); }
    [[nodiscard]] bool has_output_interval() const noexcept { return bit_.test(index_output_interval); }
    [[nodiscard]] bool has_output_param() const noexcept { return bit_.test(index_output_param); }
    [[nodiscard]] bool has_output_strategy() const noexcept { return bit_.test(index_output_strategy); }
    [[nodiscard]] bool has_downstream_info() const noexcept { return bit_.test(index_downstream_info); }
    [[nodiscard]] bool has_create_time() const noexcept { return bit_.test(index_create_time); }

    void clear() noexcept { *this = {}; }
    void clear_id() noexcept { id_ = 0; bit_.reset(index_id); }
    void clear_sensor_list_id() noexcept { sensor_list_id_ = 0; bit_.reset(index_sensor_list_id); }
    void clear_project_id() noexcept { project_id_ = 0; bit_.reset(index_project_id); }
    void clear_group_id() noexcept { group_id_ = 0; bit_.reset(index_group_id); }
    void clear_car_imei() noexcept { car_imei_ = ""; bit_.reset(index_car_imei); }
    void clear_name() noexcept { name_ = ""; bit_.reset(index_name); }
    void clear_description() noexcept { description_ = ""; bit_.reset(index_description); }
    void clear_sensor_info() noexcept { sensor_info_ = ""; bit_.reset(index_sensor_info); }
    void clear_input_enabled() noexcept { input_enabled_ = false; bit_.reset(index_input_enabled); }
    void clear_output_enabled() noexcept { output_enabled_ = false; bit_.reset(index_output_enabled); }
    void clear_downstream_enabled() noexcept { downstream_enabled_ = false; bit_.reset(index_downstream_enabled); }
    void clear_input_range_min() noexcept { input_range_min_ = 0.0; bit_.reset(index_input_range_min); }
    void clear_input_range_max() noexcept { input_range_max_ = 0.0; bit_.reset(index_input_range_max); }
    void clear_input_interval() noexcept { input_interval_ = 0; bit_.reset(index_input_interval); }
    void clear_input_param() noexcept { input_param_ = ""; bit_.reset(index_input_param); }
    void clear_input_strategy() noexcept { input_strategy_ = ""; bit_.reset(index_input_strategy); }
    void clear_output_auto_mode() noexcept { output_auto_mode_ = false; bit_.reset(index_output_auto_mode); }
    void clear_output_heartbeat_interval() noexcept { output_heartbeat_interval_ = 0; bit_.reset(index_output_heartbeat_interval); }
    void clear_output_interval() noexcept { output_interval_ = 0; bit_.reset(index_output_interval); }
    void clear_output_param() noexcept { output_param_ = ""; bit_.reset(index_output_param); }
    void clear_output_strategy() noexcept { output_strategy_ = ""; bit_.reset(index_output_strategy); }
    void clear_downstream_info() noexcept { downstream_info_ = ""; bit_.reset(index_downstream_info); }
    void clear_create_time() noexcept { create_time_ = ""; bit_.reset(index_create_time); }

    void set_id(const int64_t id) noexcept { id_ = id; bit_.set(index_id); }
    void set_sensor_list_id(const int64_t sensor_list_id) noexcept { sensor_list_id_ = sensor_list_id; bit_.set(index_sensor_list_id); }
    void set_project_id(const int64_t project_id) noexcept { project_id_ = project_id; bit_.set(index_project_id); }
    void set_group_id(const int64_t group_id) noexcept { group_id_ = group_id; bit_.set(index_group_id); }
    void set_car_imei(const char* car_imei) noexcept { car_imei_ = car_imei; bit_.set(index_car_imei); }
    void set_car_imei(const std::string_view car_imei) noexcept { car_imei_ = car_imei; bit_.set(index_car_imei); }
    void set_car_imei(const std::string& car_imei) noexcept { car_imei_ = car_imei; bit_.set(index_car_imei); }
    void set_car_imei(std::string&& car_imei) noexcept { car_imei_ = std::move(car_imei); bit_.set(index_car_imei); }
    void set_name(const char* name) noexcept { name_ = name; bit_.set(index_name); }
    void set_name(const std::string_view name) noexcept { name_ = name; bit_.set(index_name); }
    void set_name(const std::string& name) noexcept { name_ = name; bit_.set(index_name); }
    void set_name(std::string&& name) noexcept { name_ = std::move(name); bit_.set(index_name); }
    void set_description(const char* description) noexcept { description_ = description; bit_.set(index_description); }
    void set_description(const std::string_view description) noexcept { description_ = description; bit_.set(index_description); }
    void set_description(const std::string& description) noexcept { description_ = description; bit_.set(index_description); }
    void set_description(std::string&& description) noexcept { description_ = std::move(description); bit_.set(index_description); }
    void set_sensor_info(const char* sensor_info) noexcept { sensor_info_ = sensor_info; bit_.set(index_sensor_info); }
    void set_sensor_info(const std::string_view sensor_info) noexcept { sensor_info_ = sensor_info; bit_.set(index_sensor_info); }
    void set_sensor_info(const std::string& sensor_info) noexcept { sensor_info_ = sensor_info; bit_.set(index_sensor_info); }
    void set_sensor_info(std::string&& sensor_info) noexcept { sensor_info_ = std::move(sensor_info); bit_.set(index_sensor_info); }
    void set_input_enabled(const bool input_enabled) noexcept { input_enabled_ = input_enabled; bit_.set(index_input_enabled); }
    void set_output_enabled(const bool output_enabled) noexcept { output_enabled_ = output_enabled; bit_.set(index_output_enabled); }
    void set_downstream_enabled(const bool downstream_enabled) noexcept { downstream_enabled_ = downstream_enabled; bit_.set(index_downstream_enabled); }
    void set_input_range_min(const double input_range_min) noexcept { input_range_min_ = input_range_min; bit_.set(index_input_range_min); }
    void set_input_range_max(const double input_range_max) noexcept { input_range_max_ = input_range_max; bit_.set(index_input_range_max); }
    void set_input_interval(const int32_t input_interval) noexcept { input_interval_ = input_interval; bit_.set(index_input_interval); }
    void set_input_param(const char* input_param) noexcept { input_param_ = input_param; bit_.set(index_input_param); }
    void set_input_param(const std::string_view input_param) noexcept { input_param_ = input_param; bit_.set(index_input_param); }
    void set_input_param(const std::string& input_param) noexcept { input_param_ = input_param; bit_.set(index_input_param); }
    void set_input_param(std::string&& input_param) noexcept { input_param_ = std::move(input_param); bit_.set(index_input_param); }
    void set_input_strategy(const char* input_strategy) noexcept { input_strategy_ = input_strategy; bit_.set(index_input_strategy); }
    void set_input_strategy(const std::string_view input_strategy) noexcept { input_strategy_ = input_strategy; bit_.set(index_input_strategy); }
    void set_input_strategy(const std::string& input_strategy) noexcept { input_strategy_ = input_strategy; bit_.set(index_input_strategy); }
    void set_input_strategy(std::string&& input_strategy) noexcept { input_strategy_ = std::move(input_strategy); bit_.set(index_input_strategy); }
    void set_output_auto_mode(const bool output_auto_mode) noexcept { output_auto_mode_ = output_auto_mode; bit_.set(index_output_auto_mode); }
    void set_output_heartbeat_interval(const int32_t output_heartbeat_interval) noexcept { output_heartbeat_interval_ = output_heartbeat_interval; bit_.set(index_output_heartbeat_interval); }
    void set_output_interval(const int32_t output_interval) noexcept { output_interval_ = output_interval; bit_.set(index_output_interval); }
    void set_output_param(const char* output_param) noexcept { output_param_ = output_param; bit_.set(index_output_param); }
    void set_output_param(const std::string_view output_param) noexcept { output_param_ = output_param; bit_.set(index_output_param); }
    void set_output_param(const std::string& output_param) noexcept { output_param_ = output_param; bit_.set(index_output_param); }
    void set_output_param(std::string&& output_param) noexcept { output_param_ = std::move(output_param); bit_.set(index_output_param); }
    void set_output_strategy(const char* output_strategy) noexcept { output_strategy_ = output_strategy; bit_.set(index_output_strategy); }
    void set_output_strategy(const std::string_view output_strategy) noexcept { output_strategy_ = output_strategy; bit_.set(index_output_strategy); }
    void set_output_strategy(const std::string& output_strategy) noexcept { output_strategy_ = output_strategy; bit_.set(index_output_strategy); }
    void set_output_strategy(std::string&& output_strategy) noexcept { output_strategy_ = std::move(output_strategy); bit_.set(index_output_strategy); }
    void set_downstream_info(const char* downstream_info) noexcept { downstream_info_ = downstream_info; bit_.set(index_downstream_info); }
    void set_downstream_info(const std::string_view downstream_info) noexcept { downstream_info_ = downstream_info; bit_.set(index_downstream_info); }
    void set_downstream_info(const std::string& downstream_info) noexcept { downstream_info_ = downstream_info; bit_.set(index_downstream_info); }
    void set_downstream_info(std::string&& downstream_info) noexcept { downstream_info_ = std::move(downstream_info); bit_.set(index_downstream_info); }
    void set_create_time(const char* create_time) noexcept { create_time_ = create_time; bit_.set(index_create_time); }
    void set_create_time(const std::string_view create_time) noexcept { create_time_ = create_time; bit_.set(index_create_time); }
    void set_create_time(const std::string& create_time) noexcept { create_time_ = create_time; bit_.set(index_create_time); }
    void set_create_time(std::string&& create_time) noexcept { create_time_ = std::move(create_time); bit_.set(index_create_time); }

    SensorParamRow& SetValidColumns();
    SensorParamRow& SetValidColumns(const std::initializer_list<int>& valid_columns);
    SensorParamRow& SetInvalidColumns();
    void FromDatabaseRow(const pqxx::row& row);
    void FromJson(const nlohmann::json& root);
    [[nodiscard]] nlohmann::json ToJson() const;
    [[nodiscard]] std::string String(int index) const noexcept;
};

struct SensorParamTable
{
    common::ErrorCode error_code;
    std::vector<SensorParamRow> table;

    SensorParamTable() = default;
    SensorParamTable(const SensorParamTable&) = default;
    SensorParamTable(SensorParamTable&&) noexcept = default;
    SensorParamTable& operator=(const SensorParamTable&) = default;
    SensorParamTable& operator=(SensorParamTable&&) noexcept = default;

    SensorParamRow& AddRow();
    void Clear();
    [[nodiscard]] nlohmann::json ToJson() const;
};

}
