mod visibility;
mod declaration;
mod self_n_super;


pub fn test_visibility()
{
    visibility::test_visibility();
}

pub fn test_declaration() {
    declaration::test_declaration();
}

pub fn test_self_n_super() {
    self_n_super::test_self_n_super();
}