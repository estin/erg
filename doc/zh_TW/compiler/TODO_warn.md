# 警告(尚未實現)

[![badge](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com%2Fdefault%2Fsource_up_to_date%3Fowner%3Derg-lang%26repos%3Derg%26ref%3Dmain%26path%3Ddoc/EN/compiler/TODO_warn.md%26commit_hash%3Dd15cbbf7b33df0f78a575cff9679d84c36ea3ab1)](https://gezf7g7pd5.execute-api.ap-northeast-1.amazonaws.com/default/source_up_to_date?owner=erg-lang&repos=erg&ref=main&path=doc/EN/compiler/TODO_warn.md&commit_hash=d15cbbf7b33df0f78a575cff9679d84c36ea3ab1)

* `t = {(record type)}` => `T = {(record type)}`?(只有定義為常量的類型才能用于類型說明)
* `{I: Int | ...}!` => `{I: Int! | ...}`
* for/while 塊中的`return x`(`x != ()`) => `f::return`(外部塊)？