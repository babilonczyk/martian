## Martian 👾

Modular utility library centered around Mars.

## Versioning

Each minor version is a wip work on the specific module/s. It means that function signature and names may change in that stage. Eg. since Rust doesn't allow having default arguments, it may be neccessary to add one when bumping the crate.

Once the minor version gets changed to to the new one, we can consider the previous one stable. From that point forward each change to that module/s will need to have backward compatibility.

Major version is a breaking change that will make previous stable modules incompatible with new implementation.

- Version `0.1.x` - Time and Date modules

## Stable Modules

None as of yet. Currently working on `time` and `date`.

## Roadmap

If you have ideas for a valid and bringing business value modules/functions. Please create an Issue to make a discussion. Contributions are also welcome.

Time:

- [x] current_sol/msd_now
- [x] mtc_now
- [x] utc_to_msd
- [x] msd_to_utc
- [ ] darian_now (Current date based on Darian calendar)
- [ ] darian_to_utc
- [ ] utc_to_darian
- [ ] darian_to_sol
- [ ] sol_to_darian

## Licence

The library is under **Apache License, Version 2.0**
