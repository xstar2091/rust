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

class SensorListRow
{

    int64_t id_ = 0;
    int32_t version_major_ = 0;
    int32_t version_minor_ = 0;
    std::string name_;
    std::string description_;
    std::string create_time_;

    enum
    {
        index_id,
        index_version_major,
        index_version_minor,
        index_name,
        index_description,
        index_create_time,
    };
    std::bitset<index_total_count> bit_;

public:
    SensorListRow() = default;
    SensorListRow(const SensorListRow&) = default;
    SensorListRow(SensorListRow&&) noexcept = default;
    SensorListRow& operator=(const SensorListRow&) = default;
    SensorListRow& operator=(SensorListRow&&) noexcept = default;
    ~SensorListRow() = default;

    struct Column
    {
        static constexpr std::string_view id = "id"
        static constexpr std::string_view version_major = "version_major"
        static constexpr std::string_view version_minor = "version_minor"
        static constexpr std::string_view name = "name"
        static constexpr std::string_view description = "description"
        static constexpr std::string_view create_time = "create_time"
        static constexpr std::array<std::string_view, index_total_count> placeholders = {
            "$1",
            "$2",
            "$3",
            "$4",
            "$5",
            "$6",
        };
        static constexpr std::array<std::string_view, index_total_count> columns = {
            id,
            version_major,
            version_minor,
            name,
            description,
            create_time,
        };
        static constexpr int total_count = index_total_count;
    };
    static constexpr std::string_view table_name = "sensor_list";

