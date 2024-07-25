# felper
a rust console application to make certain tasks much faster! :)

# Features

> Create flutter_modular file structure

``` bash
felper modular /{file_name}

```

What does this do? This will create a folder of file_name, and create the bellow structure

```
- /file_name
    - /core
        - /bloc
            file_name_bloc.dart
            ... 
        - /widgets
            - widgets.dart
        - core.dart
    - file_name_module.dart
    - file_name_page.dart 
    - file_name.dart 
```

# Future 

Want to create a simple way of create new flutter projects with a more standardized system for mvvm.

