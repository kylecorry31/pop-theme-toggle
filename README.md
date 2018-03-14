# Pop Theme Toggle
Pop Theme Toggle is a simple script which will toggle between dark and light themes for Pop!\_OS.

## User Themes
If [GNOME User Themes](https://extensions.gnome.org/extension/19/user-themes/) extension is not installed or enabled, this script will still be able to modify the shell theme, though better support is likely possible if user-themes is installed.

## Install
Clone the GitHub repository and install using Make.

```shell
$ git clone https://github.com/kylecorry31/pop-theme-toggle
$ cd pop-theme-toggle
$ make
$ sudo make install
```

## Usage
### Switch to Dark Theme
```shell
pop-theme-toggle dark
```

### Switch to Light Theme
```shell
pop-theme-toggle light
```

### Switch to Slim Light Theme
```shell
pop-theme-toggle slim-light
```

### Switch to Slim Dark Theme
```shell
pop-theme-toggle slim-dark
```

### Toggle Between Dark and Light
```shell
pop-theme-toggle toggle
```

### Switch to Slim Theme
```shell
pop-theme-toggle slim
```

## License
This project is licensed under the [GNU General Public License v3.0](LICENSE).
