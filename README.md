# Vault Utils
Utilities to manipulate Porn Vault database.

*Always back up your config folder!*

## Parameters
- `<real root>` absolute path to the video folder on the host. e.g. `/mnt/hdd/videos`
- `<mout root>` mount path of the video folder inside the container defined in `config.json` e.g. `/videos`
- `<library path>` absolute path to the library folder. This is the config folder with `/library` on the end e.g. `/mnt/hdd/config/library`

## Commands
- `add_studios_to_scenes` if the scene name is detected in the path because PV fails to do this loads of the time
- `remove_missing_scenes` remove scene entries for files no longer on disk
- `remove_missing_labels` remove labels for scenes that no longer exist cos no cascading deletion lol

## Invocation
- `vault-utils -h` to print build in help.
- `vault-utils <real root>:<mount root>:<library path> <command>`
- `vault-utils /mnt/hdd/videos:/videos:/mnt/hdd/config/library remove_missing_scenes`
- `vault-utils /mnt/hdd/videos:/videos:/mnt/hdd/config/library remove_missing_labels`

## License
GPL3