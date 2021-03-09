# iced-viewmacro
Unofficial macro utility for iced `view` function.

## Usage
You can replace view function return value by using `view!()`.

### Before
```Rust
Column::new()
    .padding(20)
    .align_items(Align::Center)
    .push(
        Text::new(String::new("Hello, world!").size(30)
    )
)
```

### After
```Rust
view!(
    (Column::new(); padding={20} align_items={Align::Center} {
        (Text::new(String::new("Hello, world!")); size={30})
    })
)
```
