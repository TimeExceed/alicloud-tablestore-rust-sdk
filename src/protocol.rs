
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrimaryKeyType {
    INTEGER = 1,
    STRING = 2,
    BINARY = 3,
}

impl Default for PrimaryKeyType {
    fn default() -> Self {
        PrimaryKeyType::INTEGER
    }
}

impl From<i32> for PrimaryKeyType {
    fn from(i: i32) -> Self {
        match i {
            1 => PrimaryKeyType::INTEGER,
            2 => PrimaryKeyType::STRING,
            3 => PrimaryKeyType::BINARY,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for PrimaryKeyType {
    fn from(s: &'a str) -> Self {
        match s {
            "INTEGER" => PrimaryKeyType::INTEGER,
            "STRING" => PrimaryKeyType::STRING,
            "BINARY" => PrimaryKeyType::BINARY,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum PrimaryKeyOption {
    AUTO_INCREMENT = 1,
}

impl Default for PrimaryKeyOption {
    fn default() -> Self {
        PrimaryKeyOption::AUTO_INCREMENT
    }
}

impl From<i32> for PrimaryKeyOption {
    fn from(i: i32) -> Self {
        match i {
            1 => PrimaryKeyOption::AUTO_INCREMENT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for PrimaryKeyOption {
    fn from(s: &'a str) -> Self {
        match s {
            "AUTO_INCREMENT" => PrimaryKeyOption::AUTO_INCREMENT,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BloomFilterType {
    NONE = 1,
    CELL = 2,
    ROW = 3,
}

impl Default for BloomFilterType {
    fn default() -> Self {
        BloomFilterType::NONE
    }
}

impl From<i32> for BloomFilterType {
    fn from(i: i32) -> Self {
        match i {
            1 => BloomFilterType::NONE,
            2 => BloomFilterType::CELL,
            3 => BloomFilterType::ROW,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for BloomFilterType {
    fn from(s: &'a str) -> Self {
        match s {
            "NONE" => BloomFilterType::NONE,
            "CELL" => BloomFilterType::CELL,
            "ROW" => BloomFilterType::ROW,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TableStatus {
    ACTIVE = 1,
    INACTIVE = 2,
    LOADING = 3,
    UNLOADING = 4,
    UPDATING = 5,
}

impl Default for TableStatus {
    fn default() -> Self {
        TableStatus::ACTIVE
    }
}

impl From<i32> for TableStatus {
    fn from(i: i32) -> Self {
        match i {
            1 => TableStatus::ACTIVE,
            2 => TableStatus::INACTIVE,
            3 => TableStatus::LOADING,
            4 => TableStatus::UNLOADING,
            5 => TableStatus::UPDATING,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for TableStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "ACTIVE" => TableStatus::ACTIVE,
            "INACTIVE" => TableStatus::INACTIVE,
            "LOADING" => TableStatus::LOADING,
            "UNLOADING" => TableStatus::UNLOADING,
            "UPDATING" => TableStatus::UPDATING,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum RowExistenceExpectation {
    IGNORE = 0,
    EXPECT_EXIST = 1,
    EXPECT_NOT_EXIST = 2,
}

impl Default for RowExistenceExpectation {
    fn default() -> Self {
        RowExistenceExpectation::IGNORE
    }
}

impl From<i32> for RowExistenceExpectation {
    fn from(i: i32) -> Self {
        match i {
            0 => RowExistenceExpectation::IGNORE,
            1 => RowExistenceExpectation::EXPECT_EXIST,
            2 => RowExistenceExpectation::EXPECT_NOT_EXIST,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for RowExistenceExpectation {
    fn from(s: &'a str) -> Self {
        match s {
            "IGNORE" => RowExistenceExpectation::IGNORE,
            "EXPECT_EXIST" => RowExistenceExpectation::EXPECT_EXIST,
            "EXPECT_NOT_EXIST" => RowExistenceExpectation::EXPECT_NOT_EXIST,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ReturnType {
    RT_NONE = 0,
    RT_PK = 1,
}

impl Default for ReturnType {
    fn default() -> Self {
        ReturnType::RT_NONE
    }
}

impl From<i32> for ReturnType {
    fn from(i: i32) -> Self {
        match i {
            0 => ReturnType::RT_NONE,
            1 => ReturnType::RT_PK,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ReturnType {
    fn from(s: &'a str) -> Self {
        match s {
            "RT_NONE" => ReturnType::RT_NONE,
            "RT_PK" => ReturnType::RT_PK,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OperationType {
    PUT = 1,
    UPDATE = 2,
    DELETE = 3,
}

impl Default for OperationType {
    fn default() -> Self {
        OperationType::PUT
    }
}

impl From<i32> for OperationType {
    fn from(i: i32) -> Self {
        match i {
            1 => OperationType::PUT,
            2 => OperationType::UPDATE,
            3 => OperationType::DELETE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for OperationType {
    fn from(s: &'a str) -> Self {
        match s {
            "PUT" => OperationType::PUT,
            "UPDATE" => OperationType::UPDATE,
            "DELETE" => OperationType::DELETE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[warn(non_camel_case_types)]
pub enum Direction {
    FORWARD = 0,
    BACKWARD = 1,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::FORWARD
    }
}

impl From<i32> for Direction {
    fn from(i: i32) -> Self {
        match i {
            0 => Direction::FORWARD,
            1 => Direction::BACKWARD,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Direction {
    fn from(s: &'a str) -> Self {
        match s {
            "FORWARD" => Direction::FORWARD,
            "BACKWARD" => Direction::BACKWARD,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Error {
    pub code: String,
    pub message: Option<String>,
}

impl<'a> MessageRead<'a> for Error {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.code = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.message = Some(r.read_string(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Error {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.code).len())
        + self.message.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.code))?;
        if let Some(ref s) = self.message { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PrimaryKeySchema {
    pub name: String,
    pub type_pb: PrimaryKeyType,
    pub option: Option<PrimaryKeyOption>,
}

impl<'a> MessageRead<'a> for PrimaryKeySchema {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.type_pb = r.read_enum(bytes)?,
                Ok(24) => msg.option = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PrimaryKeySchema {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.name).len())
        + 1 + sizeof_varint(*(&self.type_pb) as u64)
        + self.option.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.name))?;
        w.write_with_tag(16, |w| w.write_enum(*&self.type_pb as i32))?;
        if let Some(ref s) = self.option { w.write_with_tag(24, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PartitionRange {
    pub begin: Vec<u8>,
    pub end: Vec<u8>,
}

impl<'a> MessageRead<'a> for PartitionRange {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.begin = r.read_bytes(bytes)?.to_owned(),
                Ok(18) => msg.end = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PartitionRange {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.begin).len())
        + 1 + sizeof_len((&self.end).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_bytes(&**&self.begin))?;
        w.write_with_tag(18, |w| w.write_bytes(&**&self.end))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TableOptions {
    pub time_to_live: Option<i32>,
    pub max_versions: Option<i32>,
    pub bloom_filter_type: Option<BloomFilterType>,
    pub block_size: Option<i32>,
    pub deviation_cell_version_in_sec: Option<i64>,
}

impl<'a> MessageRead<'a> for TableOptions {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.time_to_live = Some(r.read_int32(bytes)?),
                Ok(16) => msg.max_versions = Some(r.read_int32(bytes)?),
                Ok(24) => msg.bloom_filter_type = Some(r.read_enum(bytes)?),
                Ok(32) => msg.block_size = Some(r.read_int32(bytes)?),
                Ok(40) => msg.deviation_cell_version_in_sec = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TableOptions {
    fn get_size(&self) -> usize {
        0
        + self.time_to_live.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.max_versions.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.bloom_filter_type.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.block_size.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.deviation_cell_version_in_sec.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.time_to_live { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.max_versions { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.bloom_filter_type { w.write_with_tag(24, |w| w.write_enum(*s as i32))?; }
        if let Some(ref s) = self.block_size { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.deviation_cell_version_in_sec { w.write_with_tag(40, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TableMeta {
    pub table_name: String,
    pub primary_key: Vec<PrimaryKeySchema>,
}

impl<'a> MessageRead<'a> for TableMeta {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.primary_key.push(r.read_message::<PrimaryKeySchema>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TableMeta {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
        + self.primary_key.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        for s in &self.primary_key { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Condition {
    pub row_existence: RowExistenceExpectation,
    pub column_condition: Option<Vec<u8>>,
}

impl<'a> MessageRead<'a> for Condition {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.row_existence = r.read_enum(bytes)?,
                Ok(18) => msg.column_condition = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Condition {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.row_existence) as u64)
        + self.column_condition.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_enum(*&self.row_existence as i32))?;
        if let Some(ref s) = self.column_condition { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CapacityUnit {
    pub read: Option<i32>,
    pub write: Option<i32>,
}

impl<'a> MessageRead<'a> for CapacityUnit {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.read = Some(r.read_int32(bytes)?),
                Ok(16) => msg.write = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CapacityUnit {
    fn get_size(&self) -> usize {
        0
        + self.read.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.write.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.read { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.write { w.write_with_tag(16, |w| w.write_int32(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReservedThroughputDetails {
    pub capacity_unit: CapacityUnit,
    pub last_increase_time: i64,
    pub last_decrease_time: Option<i64>,
}

impl<'a> MessageRead<'a> for ReservedThroughputDetails {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.capacity_unit = r.read_message::<CapacityUnit>(bytes)?,
                Ok(16) => msg.last_increase_time = r.read_int64(bytes)?,
                Ok(24) => msg.last_decrease_time = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ReservedThroughputDetails {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.capacity_unit).get_size())
        + 1 + sizeof_varint(*(&self.last_increase_time) as u64)
        + self.last_decrease_time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.capacity_unit))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.last_increase_time))?;
        if let Some(ref s) = self.last_decrease_time { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReservedThroughput {
    pub capacity_unit: CapacityUnit,
}

impl<'a> MessageRead<'a> for ReservedThroughput {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.capacity_unit = r.read_message::<CapacityUnit>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ReservedThroughput {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.capacity_unit).get_size())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.capacity_unit))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ConsumedCapacity {
    pub capacity_unit: CapacityUnit,
}

impl<'a> MessageRead<'a> for ConsumedCapacity {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.capacity_unit = r.read_message::<CapacityUnit>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ConsumedCapacity {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.capacity_unit).get_size())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.capacity_unit))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CreateTableRequest {
    pub table_meta: TableMeta,
    pub reserved_throughput: ReservedThroughput,
    pub table_options: Option<TableOptions>,
    pub partitions: Vec<PartitionRange>,
}

impl<'a> MessageRead<'a> for CreateTableRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_meta = r.read_message::<TableMeta>(bytes)?,
                Ok(18) => msg.reserved_throughput = r.read_message::<ReservedThroughput>(bytes)?,
                Ok(26) => msg.table_options = Some(r.read_message::<TableOptions>(bytes)?),
                Ok(34) => msg.partitions.push(r.read_message::<PartitionRange>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CreateTableRequest {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_meta).get_size())
        + 1 + sizeof_len((&self.reserved_throughput).get_size())
        + self.table_options.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.partitions.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.table_meta))?;
        w.write_with_tag(18, |w| w.write_message(&self.reserved_throughput))?;
        if let Some(ref s) = self.table_options { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.partitions { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CreateTableResponse { }

impl<'a> MessageRead<'a> for CreateTableResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for CreateTableResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdateTableRequest {
    pub table_name: String,
    pub reserved_throughput: Option<ReservedThroughput>,
    pub table_options: Option<TableOptions>,
}

impl<'a> MessageRead<'a> for UpdateTableRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.reserved_throughput = Some(r.read_message::<ReservedThroughput>(bytes)?),
                Ok(26) => msg.table_options = Some(r.read_message::<TableOptions>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UpdateTableRequest {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
        + self.reserved_throughput.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.table_options.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        if let Some(ref s) = self.reserved_throughput { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.table_options { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdateTableResponse {
    pub reserved_throughput_details: ReservedThroughputDetails,
    pub table_options: TableOptions,
}

impl<'a> MessageRead<'a> for UpdateTableResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.reserved_throughput_details = r.read_message::<ReservedThroughputDetails>(bytes)?,
                Ok(18) => msg.table_options = r.read_message::<TableOptions>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UpdateTableResponse {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.reserved_throughput_details).get_size())
        + 1 + sizeof_len((&self.table_options).get_size())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.reserved_throughput_details))?;
        w.write_with_tag(18, |w| w.write_message(&self.table_options))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DescribeTableRequest {
    pub table_name: String,
}

impl<'a> MessageRead<'a> for DescribeTableRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DescribeTableRequest {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DescribeTableResponse {
    pub table_meta: TableMeta,
    pub reserved_throughput_details: ReservedThroughputDetails,
    pub table_options: TableOptions,
    pub table_status: TableStatus,
    pub shard_splits: Vec<Vec<u8>>,
}

impl<'a> MessageRead<'a> for DescribeTableResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_meta = r.read_message::<TableMeta>(bytes)?,
                Ok(18) => msg.reserved_throughput_details = r.read_message::<ReservedThroughputDetails>(bytes)?,
                Ok(26) => msg.table_options = r.read_message::<TableOptions>(bytes)?,
                Ok(32) => msg.table_status = r.read_enum(bytes)?,
                Ok(50) => msg.shard_splits.push(r.read_bytes(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DescribeTableResponse {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_meta).get_size())
        + 1 + sizeof_len((&self.reserved_throughput_details).get_size())
        + 1 + sizeof_len((&self.table_options).get_size())
        + 1 + sizeof_varint(*(&self.table_status) as u64)
        + self.shard_splits.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.table_meta))?;
        w.write_with_tag(18, |w| w.write_message(&self.reserved_throughput_details))?;
        w.write_with_tag(26, |w| w.write_message(&self.table_options))?;
        w.write_with_tag(32, |w| w.write_enum(*&self.table_status as i32))?;
        for s in &self.shard_splits { w.write_with_tag(50, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ListTableRequest { }

impl<'a> MessageRead<'a> for ListTableRequest {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ListTableRequest { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ListTableResponse {
    pub table_names: Vec<String>,
}

impl<'a> MessageRead<'a> for ListTableResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_names.push(r.read_string(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ListTableResponse {
    fn get_size(&self) -> usize {
        0
        + self.table_names.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.table_names { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteTableRequest {
    pub table_name: String,
}

impl<'a> MessageRead<'a> for DeleteTableRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DeleteTableRequest {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteTableResponse { }

impl<'a> MessageRead<'a> for DeleteTableResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for DeleteTableResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LoadTableRequest {
    pub table_name: String,
}

impl<'a> MessageRead<'a> for LoadTableRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for LoadTableRequest {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LoadTableResponse { }

impl<'a> MessageRead<'a> for LoadTableResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for LoadTableResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnloadTableRequest {
    pub table_name: String,
}

impl<'a> MessageRead<'a> for UnloadTableRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UnloadTableRequest {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnloadTableResponse { }

impl<'a> MessageRead<'a> for UnloadTableResponse {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for UnloadTableResponse { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TimeRange {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub specific_time: Option<i64>,
}

impl<'a> MessageRead<'a> for TimeRange {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.start_time = Some(r.read_int64(bytes)?),
                Ok(16) => msg.end_time = Some(r.read_int64(bytes)?),
                Ok(24) => msg.specific_time = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TimeRange {
    fn get_size(&self) -> usize {
        0
        + self.start_time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.end_time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.specific_time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.start_time { w.write_with_tag(8, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.end_time { w.write_with_tag(16, |w| w.write_int64(*s))?; }
        if let Some(ref s) = self.specific_time { w.write_with_tag(24, |w| w.write_int64(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReturnContent {
    pub return_type: Option<ReturnType>,
}

impl<'a> MessageRead<'a> for ReturnContent {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.return_type = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ReturnContent {
    fn get_size(&self) -> usize {
        0
        + self.return_type.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.return_type { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetRowRequest {
    pub table_name: String,
    pub primary_key: Vec<u8>,
    pub columns_to_get: Vec<String>,
    pub time_range: Option<TimeRange>,
    pub max_versions: Option<i32>,
    pub cache_blocks: bool,
    pub filter: Option<Vec<u8>>,
    pub start_column: Option<String>,
    pub end_column: Option<String>,
    pub token: Option<Vec<u8>>,
}

impl<'a> MessageRead<'a> for GetRowRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = GetRowRequest {
            cache_blocks: true,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.primary_key = r.read_bytes(bytes)?.to_owned(),
                Ok(26) => msg.columns_to_get.push(r.read_string(bytes)?.to_owned()),
                Ok(34) => msg.time_range = Some(r.read_message::<TimeRange>(bytes)?),
                Ok(40) => msg.max_versions = Some(r.read_int32(bytes)?),
                Ok(48) => msg.cache_blocks = r.read_bool(bytes)?,
                Ok(58) => msg.filter = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(66) => msg.start_column = Some(r.read_string(bytes)?.to_owned()),
                Ok(74) => msg.end_column = Some(r.read_string(bytes)?.to_owned()),
                Ok(82) => msg.token = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetRowRequest {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
        + 1 + sizeof_len((&self.primary_key).len())
        + self.columns_to_get.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.time_range.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.max_versions.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + if self.cache_blocks == true { 0 } else { 1 + sizeof_varint(*(&self.cache_blocks) as u64) }
        + self.filter.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.start_column.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.end_column.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.token.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        w.write_with_tag(18, |w| w.write_bytes(&**&self.primary_key))?;
        for s in &self.columns_to_get { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.time_range { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.max_versions { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if self.cache_blocks != true { w.write_with_tag(48, |w| w.write_bool(*&self.cache_blocks))?; }
        if let Some(ref s) = self.filter { w.write_with_tag(58, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.start_column { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.end_column { w.write_with_tag(74, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.token { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetRowResponse {
    pub consumed: ConsumedCapacity,
    pub row: Vec<u8>,
    pub next_token: Option<Vec<u8>>,
}

impl<'a> MessageRead<'a> for GetRowResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.consumed = r.read_message::<ConsumedCapacity>(bytes)?,
                Ok(18) => msg.row = r.read_bytes(bytes)?.to_owned(),
                Ok(26) => msg.next_token = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetRowResponse {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.consumed).get_size())
        + 1 + sizeof_len((&self.row).len())
        + self.next_token.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.consumed))?;
        w.write_with_tag(18, |w| w.write_bytes(&**&self.row))?;
        if let Some(ref s) = self.next_token { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdateRowRequest {
    pub table_name: String,
    pub row_change: Vec<u8>,
    pub condition: Condition,
    pub return_content: Option<ReturnContent>,
}

impl<'a> MessageRead<'a> for UpdateRowRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.row_change = r.read_bytes(bytes)?.to_owned(),
                Ok(26) => msg.condition = r.read_message::<Condition>(bytes)?,
                Ok(34) => msg.return_content = Some(r.read_message::<ReturnContent>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UpdateRowRequest {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
        + 1 + sizeof_len((&self.row_change).len())
        + 1 + sizeof_len((&self.condition).get_size())
        + self.return_content.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        w.write_with_tag(18, |w| w.write_bytes(&**&self.row_change))?;
        w.write_with_tag(26, |w| w.write_message(&self.condition))?;
        if let Some(ref s) = self.return_content { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdateRowResponse {
    pub consumed: ConsumedCapacity,
    pub row: Option<Vec<u8>>,
}

impl<'a> MessageRead<'a> for UpdateRowResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.consumed = r.read_message::<ConsumedCapacity>(bytes)?,
                Ok(18) => msg.row = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UpdateRowResponse {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.consumed).get_size())
        + self.row.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.consumed))?;
        if let Some(ref s) = self.row { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PutRowRequest {
    pub table_name: String,
    pub row: Vec<u8>,
    pub condition: Condition,
    pub return_content: Option<ReturnContent>,
}

impl<'a> MessageRead<'a> for PutRowRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.row = r.read_bytes(bytes)?.to_owned(),
                Ok(26) => msg.condition = r.read_message::<Condition>(bytes)?,
                Ok(34) => msg.return_content = Some(r.read_message::<ReturnContent>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PutRowRequest {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
        + 1 + sizeof_len((&self.row).len())
        + 1 + sizeof_len((&self.condition).get_size())
        + self.return_content.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        w.write_with_tag(18, |w| w.write_bytes(&**&self.row))?;
        w.write_with_tag(26, |w| w.write_message(&self.condition))?;
        if let Some(ref s) = self.return_content { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PutRowResponse {
    pub consumed: ConsumedCapacity,
    pub row: Option<Vec<u8>>,
}

impl<'a> MessageRead<'a> for PutRowResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.consumed = r.read_message::<ConsumedCapacity>(bytes)?,
                Ok(18) => msg.row = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PutRowResponse {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.consumed).get_size())
        + self.row.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.consumed))?;
        if let Some(ref s) = self.row { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteRowRequest {
    pub table_name: String,
    pub primary_key: Vec<u8>,
    pub condition: Condition,
    pub return_content: Option<ReturnContent>,
}

impl<'a> MessageRead<'a> for DeleteRowRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.primary_key = r.read_bytes(bytes)?.to_owned(),
                Ok(26) => msg.condition = r.read_message::<Condition>(bytes)?,
                Ok(34) => msg.return_content = Some(r.read_message::<ReturnContent>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DeleteRowRequest {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
        + 1 + sizeof_len((&self.primary_key).len())
        + 1 + sizeof_len((&self.condition).get_size())
        + self.return_content.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        w.write_with_tag(18, |w| w.write_bytes(&**&self.primary_key))?;
        w.write_with_tag(26, |w| w.write_message(&self.condition))?;
        if let Some(ref s) = self.return_content { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeleteRowResponse {
    pub consumed: ConsumedCapacity,
    pub row: Option<Vec<u8>>,
}

impl<'a> MessageRead<'a> for DeleteRowResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.consumed = r.read_message::<ConsumedCapacity>(bytes)?,
                Ok(18) => msg.row = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DeleteRowResponse {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.consumed).get_size())
        + self.row.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.consumed))?;
        if let Some(ref s) = self.row { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TableInBatchGetRowRequest {
    pub table_name: String,
    pub primary_key: Vec<Vec<u8>>,
    pub token: Vec<Vec<u8>>,
    pub columns_to_get: Vec<String>,
    pub time_range: Option<TimeRange>,
    pub max_versions: Option<i32>,
    pub cache_blocks: bool,
    pub filter: Option<Vec<u8>>,
    pub start_column: Option<String>,
    pub end_column: Option<String>,
}

impl<'a> MessageRead<'a> for TableInBatchGetRowRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = TableInBatchGetRowRequest {
            cache_blocks: true,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.primary_key.push(r.read_bytes(bytes)?.to_owned()),
                Ok(26) => msg.token.push(r.read_bytes(bytes)?.to_owned()),
                Ok(34) => msg.columns_to_get.push(r.read_string(bytes)?.to_owned()),
                Ok(42) => msg.time_range = Some(r.read_message::<TimeRange>(bytes)?),
                Ok(48) => msg.max_versions = Some(r.read_int32(bytes)?),
                Ok(56) => msg.cache_blocks = r.read_bool(bytes)?,
                Ok(66) => msg.filter = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(74) => msg.start_column = Some(r.read_string(bytes)?.to_owned()),
                Ok(82) => msg.end_column = Some(r.read_string(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TableInBatchGetRowRequest {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
        + self.primary_key.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.token.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.columns_to_get.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.time_range.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.max_versions.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + if self.cache_blocks == true { 0 } else { 1 + sizeof_varint(*(&self.cache_blocks) as u64) }
        + self.filter.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.start_column.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.end_column.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        for s in &self.primary_key { w.write_with_tag(18, |w| w.write_bytes(&**s))?; }
        for s in &self.token { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        for s in &self.columns_to_get { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.time_range { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.max_versions { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if self.cache_blocks != true { w.write_with_tag(56, |w| w.write_bool(*&self.cache_blocks))?; }
        if let Some(ref s) = self.filter { w.write_with_tag(66, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.start_column { w.write_with_tag(74, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.end_column { w.write_with_tag(82, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BatchGetRowRequest {
    pub tables: Vec<TableInBatchGetRowRequest>,
}

impl<'a> MessageRead<'a> for BatchGetRowRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.tables.push(r.read_message::<TableInBatchGetRowRequest>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BatchGetRowRequest {
    fn get_size(&self) -> usize {
        0
        + self.tables.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.tables { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RowInBatchGetRowResponse {
    pub is_ok: bool,
    pub error: Option<Error>,
    pub consumed: Option<ConsumedCapacity>,
    pub row: Option<Vec<u8>>,
    pub next_token: Option<Vec<u8>>,
}

impl<'a> MessageRead<'a> for RowInBatchGetRowResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.is_ok = r.read_bool(bytes)?,
                Ok(18) => msg.error = Some(r.read_message::<Error>(bytes)?),
                Ok(26) => msg.consumed = Some(r.read_message::<ConsumedCapacity>(bytes)?),
                Ok(34) => msg.row = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(42) => msg.next_token = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RowInBatchGetRowResponse {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.is_ok) as u64)
        + self.error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.consumed.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.row.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.next_token.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_bool(*&self.is_ok))?;
        if let Some(ref s) = self.error { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.consumed { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.row { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.next_token { w.write_with_tag(42, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TableInBatchGetRowResponse {
    pub table_name: String,
    pub rows: Vec<RowInBatchGetRowResponse>,
}

impl<'a> MessageRead<'a> for TableInBatchGetRowResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.rows.push(r.read_message::<RowInBatchGetRowResponse>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TableInBatchGetRowResponse {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
        + self.rows.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        for s in &self.rows { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BatchGetRowResponse {
    pub tables: Vec<TableInBatchGetRowResponse>,
}

impl<'a> MessageRead<'a> for BatchGetRowResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.tables.push(r.read_message::<TableInBatchGetRowResponse>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BatchGetRowResponse {
    fn get_size(&self) -> usize {
        0
        + self.tables.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.tables { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RowInBatchWriteRowRequest {
    pub type_pb: OperationType,
    pub row_change: Vec<u8>,
    pub condition: Condition,
    pub return_content: Option<ReturnContent>,
}

impl<'a> MessageRead<'a> for RowInBatchWriteRowRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_enum(bytes)?,
                Ok(18) => msg.row_change = r.read_bytes(bytes)?.to_owned(),
                Ok(26) => msg.condition = r.read_message::<Condition>(bytes)?,
                Ok(34) => msg.return_content = Some(r.read_message::<ReturnContent>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RowInBatchWriteRowRequest {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.type_pb) as u64)
        + 1 + sizeof_len((&self.row_change).len())
        + 1 + sizeof_len((&self.condition).get_size())
        + self.return_content.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_enum(*&self.type_pb as i32))?;
        w.write_with_tag(18, |w| w.write_bytes(&**&self.row_change))?;
        w.write_with_tag(26, |w| w.write_message(&self.condition))?;
        if let Some(ref s) = self.return_content { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TableInBatchWriteRowRequest {
    pub table_name: String,
    pub rows: Vec<RowInBatchWriteRowRequest>,
}

impl<'a> MessageRead<'a> for TableInBatchWriteRowRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.rows.push(r.read_message::<RowInBatchWriteRowRequest>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TableInBatchWriteRowRequest {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
        + self.rows.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        for s in &self.rows { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BatchWriteRowRequest {
    pub tables: Vec<TableInBatchWriteRowRequest>,
}

impl<'a> MessageRead<'a> for BatchWriteRowRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.tables.push(r.read_message::<TableInBatchWriteRowRequest>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BatchWriteRowRequest {
    fn get_size(&self) -> usize {
        0
        + self.tables.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.tables { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RowInBatchWriteRowResponse {
    pub is_ok: bool,
    pub error: Option<Error>,
    pub consumed: Option<ConsumedCapacity>,
    pub row: Option<Vec<u8>>,
}

impl<'a> MessageRead<'a> for RowInBatchWriteRowResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.is_ok = r.read_bool(bytes)?,
                Ok(18) => msg.error = Some(r.read_message::<Error>(bytes)?),
                Ok(26) => msg.consumed = Some(r.read_message::<ConsumedCapacity>(bytes)?),
                Ok(34) => msg.row = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RowInBatchWriteRowResponse {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.is_ok) as u64)
        + self.error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.consumed.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.row.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_bool(*&self.is_ok))?;
        if let Some(ref s) = self.error { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.consumed { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.row { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct TableInBatchWriteRowResponse {
    pub table_name: String,
    pub rows: Vec<RowInBatchWriteRowResponse>,
}

impl<'a> MessageRead<'a> for TableInBatchWriteRowResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.rows.push(r.read_message::<RowInBatchWriteRowResponse>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TableInBatchWriteRowResponse {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
        + self.rows.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        for s in &self.rows { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BatchWriteRowResponse {
    pub tables: Vec<TableInBatchWriteRowResponse>,
}

impl<'a> MessageRead<'a> for BatchWriteRowResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.tables.push(r.read_message::<TableInBatchWriteRowResponse>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BatchWriteRowResponse {
    fn get_size(&self) -> usize {
        0
        + self.tables.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.tables { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetRangeRequest {
    pub table_name: String,
    pub direction: Direction,
    pub columns_to_get: Vec<String>,
    pub time_range: Option<TimeRange>,
    pub max_versions: Option<i32>,
    pub limit: Option<i32>,
    pub inclusive_start_primary_key: Vec<u8>,
    pub exclusive_end_primary_key: Vec<u8>,
    pub cache_blocks: bool,
    pub filter: Option<Vec<u8>>,
    pub start_column: Option<String>,
    pub end_column: Option<String>,
    pub token: Option<Vec<u8>>,
}

impl<'a> MessageRead<'a> for GetRangeRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = GetRangeRequest {
            cache_blocks: true,
            ..Self::default()
        };
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.direction = r.read_enum(bytes)?,
                Ok(26) => msg.columns_to_get.push(r.read_string(bytes)?.to_owned()),
                Ok(34) => msg.time_range = Some(r.read_message::<TimeRange>(bytes)?),
                Ok(40) => msg.max_versions = Some(r.read_int32(bytes)?),
                Ok(48) => msg.limit = Some(r.read_int32(bytes)?),
                Ok(58) => msg.inclusive_start_primary_key = r.read_bytes(bytes)?.to_owned(),
                Ok(66) => msg.exclusive_end_primary_key = r.read_bytes(bytes)?.to_owned(),
                Ok(72) => msg.cache_blocks = r.read_bool(bytes)?,
                Ok(82) => msg.filter = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(90) => msg.start_column = Some(r.read_string(bytes)?.to_owned()),
                Ok(98) => msg.end_column = Some(r.read_string(bytes)?.to_owned()),
                Ok(106) => msg.token = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetRangeRequest {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
        + 1 + sizeof_varint(*(&self.direction) as u64)
        + self.columns_to_get.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.time_range.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.max_versions.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.limit.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + 1 + sizeof_len((&self.inclusive_start_primary_key).len())
        + 1 + sizeof_len((&self.exclusive_end_primary_key).len())
        + if self.cache_blocks == true { 0 } else { 1 + sizeof_varint(*(&self.cache_blocks) as u64) }
        + self.filter.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.start_column.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.end_column.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.token.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        w.write_with_tag(16, |w| w.write_enum(*&self.direction as i32))?;
        for s in &self.columns_to_get { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.time_range { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.max_versions { w.write_with_tag(40, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.limit { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        w.write_with_tag(58, |w| w.write_bytes(&**&self.inclusive_start_primary_key))?;
        w.write_with_tag(66, |w| w.write_bytes(&**&self.exclusive_end_primary_key))?;
        if self.cache_blocks != true { w.write_with_tag(72, |w| w.write_bool(*&self.cache_blocks))?; }
        if let Some(ref s) = self.filter { w.write_with_tag(82, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.start_column { w.write_with_tag(90, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.end_column { w.write_with_tag(98, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.token { w.write_with_tag(106, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GetRangeResponse {
    pub consumed: ConsumedCapacity,
    pub rows: Vec<u8>,
    pub next_start_primary_key: Option<Vec<u8>>,
    pub next_token: Option<Vec<u8>>,
}

impl<'a> MessageRead<'a> for GetRangeResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.consumed = r.read_message::<ConsumedCapacity>(bytes)?,
                Ok(18) => msg.rows = r.read_bytes(bytes)?.to_owned(),
                Ok(26) => msg.next_start_primary_key = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(34) => msg.next_token = Some(r.read_bytes(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GetRangeResponse {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.consumed).get_size())
        + 1 + sizeof_len((&self.rows).len())
        + self.next_start_primary_key.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.next_token.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.consumed))?;
        w.write_with_tag(18, |w| w.write_bytes(&**&self.rows))?;
        if let Some(ref s) = self.next_start_primary_key { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        if let Some(ref s) = self.next_token { w.write_with_tag(34, |w| w.write_bytes(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ComputeSplitPointsBySizeRequest {
    pub table_name: String,
    pub split_size: i64,
}

impl<'a> MessageRead<'a> for ComputeSplitPointsBySizeRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.table_name = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.split_size = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ComputeSplitPointsBySizeRequest {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.table_name).len())
        + 1 + sizeof_varint(*(&self.split_size) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.table_name))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.split_size))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ComputeSplitPointsBySizeResponse {
    pub consumed: ConsumedCapacity,
    pub schema: Vec<PrimaryKeySchema>,
    pub split_points: Vec<Vec<u8>>,
    pub locations: Vec<mod_ComputeSplitPointsBySizeResponse::SplitLocation>,
}

impl<'a> MessageRead<'a> for ComputeSplitPointsBySizeResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.consumed = r.read_message::<ConsumedCapacity>(bytes)?,
                Ok(18) => msg.schema.push(r.read_message::<PrimaryKeySchema>(bytes)?),
                Ok(26) => msg.split_points.push(r.read_bytes(bytes)?.to_owned()),
                Ok(34) => msg.locations.push(r.read_message::<mod_ComputeSplitPointsBySizeResponse::SplitLocation>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ComputeSplitPointsBySizeResponse {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.consumed).get_size())
        + self.schema.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.split_points.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.locations.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.consumed))?;
        for s in &self.schema { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.split_points { w.write_with_tag(26, |w| w.write_bytes(&**s))?; }
        for s in &self.locations { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(non_snake_case)]
pub mod mod_ComputeSplitPointsBySizeResponse {

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SplitLocation {
    pub location: String,
    pub repeat: i64,
}

impl<'a> MessageRead<'a> for SplitLocation {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.location = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.repeat = r.read_sint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SplitLocation {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.location).len())
        + 1 + sizeof_sint64(*(&self.repeat))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.location))?;
        w.write_with_tag(16, |w| w.write_sint64(*&self.repeat))?;
        Ok(())
    }
}

}

