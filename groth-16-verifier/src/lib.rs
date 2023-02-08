#![no_std]
#![feature(default_alloc_error_handler)]
use crate::key_wrap::build_vk;
use ark_bls12_377::Bls12_377;
use core::str::FromStr;
use soroban_sdk::{contractimpl, symbol, vec, Env, Symbol, Vec};

extern crate alloc;

struct MyAllocator;

unsafe impl alloc::alloc::GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: alloc::alloc::Layout) -> *mut u8 {
        alloc::alloc::alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::alloc::Layout) {
        alloc::alloc::dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

pub struct TestVerifier;

#[contractimpl]
impl TestVerifier {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }

    pub fn verify(env: Env) -> bool {
        use crate::proof_wrap::build_proof;
        use crate::verify_utils::{prepare_vk, verify};
        use ark_bls12_377::Fr;
        use ark_ff::Fp256;

        let vk = build_vk::<Bls12_377>(
	&["60056667534097566763494875577637444127327577125667378395104360090433601281821073015262935791939615206969974985807", "256966415054552652239055381347881628962687543133419319297247837226639466725550973295768665146208628522797884612532"],
(
    ["179326101817067563391800226192053756403416478052315344001456330157723906474308378357131917251349287820912488875216", "142772474757114394044991647750617200906283502485014337053255339119209019009345711113533581456207046253540462419113"],
    ["57823623095007362049286465621251671686016968923028753473912881627831409733399159258656322440482090204289401420390", "101118882174843321234893666313995678452697688295558067092605177821804773495091187819872483394205519149832113555085"],
),
(
    ["11617907731908082484305466563421494238804933052187619275134676452269894898983076561079708969471115033175497288840", "99276406171810723790445065322329090496759111597631471839516749021231950281241339835395303648287892964833344966582"],
    ["28377030248292282779224599960559433983389034648517859473214048813852768827884198220249094226263418918225188281911", "40356019344268462764549053614229069555213057348469458459409229009875049976022708987189332901989722293448063024604"],
),
(
    ["42336115354986449563222217370661181929642848266194078818577707312806800307086036834098914403582575814880913453381", "13628021446593238911524401064612849739744972041508021087657752045657598279495029493256485816948094524005775965340"],
    ["151484706494197135690423956864104880934167257664828219195051502542912986111558842872420005005007856459649370787395", "106698378680465405979799355931100154289889369263695704489508189977674129976186904452368213544558823954831378219999"]
),&[
    [
	alloc::string::String::from("49480137876630118721293405704501976762569109511907182279349390514709028541679857128154384581593933158967873027339"),
	alloc::string::String::from("74751313900017928195033612718777627165530061161956172715617292616123988275146405258067008099746094173381613927388")],
    [
	alloc::string::String::from("46034050131129928737768654039120663979581883678390123572244352208739077146930962284601941313470678196963319006322"),
	alloc::string::String::from("63494088474199393497078154058049957233489326998063576055369935194683236608833107071159288686514522146509792946199")], ]
    );
        let prep_vk = prepare_vk(&vk);
        let proof = build_proof::<Bls12_377>(
        &["15418665600506652100561046515367875140419289995947193402561019537241872027964827224166724343705483346143470852731", "167008803036115107405418679490035202868478870472330242204001273757458478058714648975048495328133467760891608546148"],
(
    ["166577210860431387619669372656358383974101276735197412785209096541557319817228478432621330273001881927051445417406", "206343684092716339277019987836299276044717244419460989968801009284705815340411102589748352273738828474466157094245"],
    ["221648819763740803203047255262280421402962307605247492463898458320485549262445592950081729098224237965423853570558", "98141685387324061212822747953034151018463609351451484713827787188859406641166893569975083223024556896317502938904"],
),
&["244220262963769885808961443982221174762003229929972154576507103882497536855742220170129599313242682265347284028286", "48509848903788445930476153095638029129985072454629825916006466567203396769632018246628830229692992210820821493168"],
    );

        let image: Fr = Fp256::from_str("4").unwrap();
        verify(proof, prep_vk, &[image])
    }
}

mod key_wrap;
mod proof_wrap;
//mod test;
mod types;
mod verify_utils;
