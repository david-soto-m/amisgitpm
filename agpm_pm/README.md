# agpm_pm

This crate implements a the three project manager traits `PMOperations`,
`PMInteractive` and `PMProgrammatic` from the `amisgitpm` crate.

They are implemented `PrjManager` struct. It is as genenric as posible,
depending exclusively on the traits defined in the `amisgitpm` crate

This crate also defines another trait,`Interactions`. This trait is used in
the `PMInteractive` implement
