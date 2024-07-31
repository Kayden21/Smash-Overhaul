#![feature(concat_idents, proc_macro_hygiene)]
#![allow(unused_macros)]

mod captain;
mod chrom;
mod custom;
mod daisy;
mod diddy;
mod donkey;
mod falco;
mod fox;
mod gamewatch;
mod ganon;
mod ike;
mod kirby;
mod koopa;
mod link;
mod lucas;
mod lucina;
mod luigi;
mod mario;
mod mariod;
mod marth;
mod metaknight;
mod mewtwo;
mod nana;
mod ness;
mod peach;
mod pfushigisou;
mod pichu;
mod pikachu;
mod pit;
mod pitb;
mod plizardon;
mod popo;
mod ptrainer;
mod purin;
mod pzenigame;
mod roy;
mod samus;
mod samusd;
mod sheik;
mod snake;
mod sonic;
mod szerosuit;
mod toonlink;
mod wario;
mod wolf;
mod yoshi;
mod younglink;
mod zelda;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    mario::install();
    donkey::install();
    mariod::install();
    samus::install();
    samusd::install();
    link::install();
    toonlink::install();
    younglink::install();
    yoshi::install();
    kirby::install();
    fox::install();
    falco::install();
    wolf::install();
    pikachu::install();
    pichu::install();
    luigi::install();
    ness::install();
    lucas::install();
    captain::install();
    ganon::install();
    purin::install();
    peach::install();
    daisy::install();
    koopa::install();
    popo::install();
    nana::install();
    sheik::install();
    zelda::install();
    marth::install();
    lucina::install();
    mewtwo::install();
    roy::install();
    chrom::install();
    gamewatch::install();
    metaknight::install();
    pit::install();
    pitb::install();
    szerosuit::install();
    wario::install();
    snake::install();
    ike::install();
    ptrainer::install();
    pzenigame::install();
    pfushigisou::install();
    plizardon::install();
    diddy::install();
    sonic::install();
    custom::install();
}