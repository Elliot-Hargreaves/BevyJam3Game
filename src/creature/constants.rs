
enum Malus {
    LoseADie,
    LoseHealth(u32),
    HalfDamage,

}

enum Elements {
    Fire,
    Poison,
    Ice,
    Electricity
}

enum Bonus {
    DoubleDamage,
    ElementalDamage(Elements),
    ExtraDie,
    ExtraEffectChoice
}

struct Effect {
    malus: Malus,
    bonus: Bonus
}

struct Effects {
    this_turns_effects: Vec<Effect>,
    next_turns_effects: Vec<Effect>
}