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

    [[nodiscard]] int64_t id() const noexcept { return id_; }
    [[nodiscard]] int32_t version_major() const noexcept { return version_major_; }
    [[nodiscard]] int32_t version_minor() const noexcept { return version_minor_; }
    [[nodiscard]] const std::string& name() const noexcept { return name_; }
    [[nodiscard]] const std::string& description() const noexcept { return description_; }
    [[nodiscard]] const std::string& create_time() const noexcept { return create_time_; }

    [[nodiscard]] bool has(const int index) const noexcept { return bit_.test(index); }
    [[nodiscard]] bool has_none() const noexcept { return bit_.none(); }
    [[nodiscard]] bool has_all() const noexcept { return bit_.all(); }
    [[nodiscard]] bool has_any() const noexcept { return bit_.any(); }
    [[nodiscard]] bool has_id() const noexcept { return bit_.test(index_id); }
    [[nodiscard]] bool has_version_major() const noexcept { return bit_.test(index_version_major); }
    [[nodiscard]] bool has_version_minor() const noexcept { return bit_.test(index_version_minor); }
    [[nodiscard]] bool has_name() const noexcept { return bit_.test(index_name); }
    [[nodiscard]] bool has_description() const noexcept { return bit_.test(index_description); }
    [[nodiscard]] bool has_create_time() const noexcept { return bit_.test(index_create_time); }

    void clear() noexcept { *this = {}; }
    void clear_id() noexcept { id_ = 0; bit_.reset(index_id); }
    void clear_version_major() noexcept { version_major_ = 0; bit_.reset(index_version_major); }
    void clear_version_minor() noexcept { version_minor_ = 0; bit_.reset(index_version_minor); }
    void clear_name() noexcept { name_ = ""; bit_.reset(index_name); }
    void clear_description() noexcept { description_ = ""; bit_.reset(index_description); }
    void clear_create_time() noexcept { create_time_ = ""; bit_.reset(index_create_time); }

    void set_id(const int64_t id) noexcept { id_ = id; bit_.set(index_id); }
    void set_version_major(const int32_t version_major) noexcept { version_major_ = version_major; bit_.set(index_version_major); }
    void set_version_minor(const int32_t version_minor) noexcept { version_minor_ = version_minor; bit_.set(index_version_minor); }
    void set_name(const char* name) noexcept { name_ = name; bit_.set(index_name); }
    void set_name(const std::string_view name) noexcept { name_ = name; bit_.set(index_name); }
    void set_name(const std::string& name) noexcept { name_ = name; bit_.set(index_name); }
    void set_name(std::string&& name) noexcept { name_ = std::move(name); bit_.set(index_name); }
    void set_description(const char* description) noexcept { description_ = description; bit_.set(index_description); }
    void set_description(const std::string_view description) noexcept { description_ = description; bit_.set(index_description); }
    void set_description(const std::string& description) noexcept { description_ = description; bit_.set(index_description); }
    void set_description(std::string&& description) noexcept { description_ = std::move(description); bit_.set(index_description); }
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

struct SensorListTable
{
    common::ErrorCode error_code;
    std::vector<SensorListRow> table;

    SensorListTable() = default;
    SensorListTable(const SensorListTable&) = default;
    SensorListTable(SensorListTable&&) noexcept = default;
    SensorListTable& operator=(const SensorListTable&) = default;
    SensorListTable& operator=(SensorListTable&&) noexcept = default;

    SensorListRow& AddRow();
    void Clear();
    [[nodiscard]] nlohmann::json ToJson() const;
};

}
