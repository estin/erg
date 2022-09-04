# Set

一个Set代表一个集合，它在结构上是一个重复的无序数组。

```python
assert Set.from([1, 2, 3, 2, 1]) == {1, 2, 3}
assert {1, 2} == {1, 1, 2} # 重复的被自动删除
assert {1, 2} == {2, 1}
```

Set可以执行集合操作。

```python
assert 1 in {1, 2, 3}
assert not 1 in {}
assert {1} or {2} == {1, 2}
assert {1, 2} and {2, 3} == {2}
assert {1, 2} not {2} == {1}
```

Set是同质集合。 为了使不同类的对象共存，它们必须同质化

```python
s: {Int or Str} = {"a", 1, "b", -1}
```

## Sets为类型
Sets也可以被视为类型。 这种类型称为 _枚举类型_。

```python
i: {1, 2, 3} = 1
assert i in {1, 2, 3}
```

Set的元素直接是类型的元素。
请注意，这些Set本身是不同的。

```python
mut_set = {1, 2, 3}.into {Int; !3}
mut_set.insert!(4)
```

<p align='center'>
    <a href='./13_record.md'>上一页</a> | <a href='./15_type.md'>下一页</a>
</p>