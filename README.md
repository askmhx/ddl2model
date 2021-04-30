# ddl2model 

##### 通过数据源生成各语言的对象/结构

支持***Go/Rust/Java/Protobuf***

表名默认去了一个前缀***'T_'***，如无需或者其他前缀请自行改代码。

编译之后生成一个可执行文件***d2m***

参数说明：

> 1. 输出目录
> 2. 数据源
> 3. 数据库名用#分隔
> 4. 语言类型GO/Rust/Java/Protobuf


示例

> d2m /Users/Crazz/Desktop root:password@localhost:3306 DB1#DB2#DB2 java
