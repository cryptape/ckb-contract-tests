# CKB Script IPC 测试用例集

## 1. 错误处理场景测试

### 1.1 基础错误处理
- **测试名称**: test_basic_error_handling
- **测试目的**: 验证基本的错误处理机制
- **测试步骤**:
  1. 部署服务端合约
  2. 客户端发起无效请求
  3. 验证错误响应
- **预期结果**: 返回对应的错误码
- **对应测试函数**: test_error_handling_invalid_request

### 1.2 系统错误处理
- **测试名称**: test_sys_error_handling
- **测试目的**: 验证CKB系统错误处理
- **测试步骤**:
  1. 模拟系统错误场景
  2. 验证错误处理
- **预期结果**: 返回CkbSysError
- **对应测试函数**: test_ipc_service_unavailable

### 1.3 序列化错误处理
- **测试名称**: test_serialization_error
- **测试目的**: 验证序列化/反序列化错误处理
- **测试步骤**:
  1. 发送无效格式数据
  2. 验证错误处理
- **预期结果**: 返回SerializeError/DeserializeError
- **对应测试函数**: test_json_serialization

## 2. 数据类型测试

### 2.1 基础类型测试
- **测试名称**: test_primitive_types_handling
- **测试目的**: 验证基础数据类型处理
- **测试步骤**:
  1. 测试所有基础类型参数
  2. 验证处理结果
- **预期结果**: 所有类型正确处理
- **对应测试函数**: test_parse_method_with_return_type

### 2.2 复杂类型测试
- **测试名称**: test_complex_types_handling
- **测试目的**: 验证复杂数据结构处理
- **测试步骤**:
  1. 测试Vec、BTreeMap等复杂类型
  2. 验证处理结果
- **预期结果**: 复杂类型正确处理
- **对应测试函数**: test_json_serialization

## 3. 极限值测试

### 3.1 大数据测试
- **测试名称**: test_large_data_handling
- **测试目的**: 验证大数据处理能力
- **测试步骤**:
  1. 发送超大数据
  2. 验证处理结果
- **预期结果**: 正确处理或返回适当错误
- **对应测试函数**: test_performance_multiple_calls

### 3.2 边界值测试
- **测试名称**: test_boundary_values
- **测试目的**: 验证边界值处理
- **测试步骤**:
  1. 测试最大/最小值
  2. 验证处理结果
- **预期结果**: 正确处理边界情况
- **对应测试函数**: test_performance_multiple_calls

## 4. 链式调用测试

### 4.1 基础链式调用
- **测试名称**: test_basic_chain_call
- **测试目的**: 验证简单链式调用
- **测试步骤**:
  1. 部署多个合约
  2. 执行链式调用
- **预期结果**: 调用链正确执行
- **对应测试函数**: test_ipc_multi_contract_interaction

### 4.2 复杂链式调用
- **测试名称**: test_complex_chain_call
- **测试目的**: 验证复杂链式调用场景
- **测试步骤**:
  1. 部署多层合约
  2. 执行复杂调用链
- **预期结果**: 复杂调用链正确执行
- **对应测试函数**: test_ipc_multi_contract_interaction

### 4.3 链式错误传播
- **测试名称**: test_chain_error_propagation
- **测试目的**: 验证错误在调用链中的传播
- **测试步骤**:
  1. 在调用链中触发错误
  2. 验证错误传播
- **预期结果**: 错误正确传播到调用方
- **对应测试函数**: test_error_handling_invalid_request

## 5. 单次服务模式测试

### 5.1 单次服务测试
- **测试名称**: test_single_service
- **测试目的**: 验证单次服务模式的正确性
- **测试步骤**:
  1. 部署服务合约
  2. 执行单次服务调用
  3. 验证服务响应
- **预期结果**: 服务正确响应并结束
- **对应测试函数**: test_single_service_mode