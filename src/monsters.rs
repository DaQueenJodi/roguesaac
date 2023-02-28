pub enum MonsterFlavor {
	Spider
}

pub trait Monster {
	fn flavor() -> MonsterFlavor;
}

