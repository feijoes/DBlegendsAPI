# Instructions for running locally

###  Prerequisites

Make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Getting Started 

1. Clone the repo 

2. generate the characters.json file in scraper

3. go to /api/ run with `cargo run`


# API Documentation

#### Current Entities

`Character`
```rs
struct Character {
    name: String,
    id: String,
    color: Color,
    rarity: Rarity,
    tags: Vec<String>,
    main_ability: MainAbility,
    unique_ability: UniqueAbilities,
    ultra_ability: Option<UltraAbility>,
    base_stats: Stats,
    max_stats: Stats,
    strike: String,
    shot: String,
    image_url: String,
    special_move: SpecialMove,
    special_skill: SpecialSkill,
    ultimate_skill: Option<UltimateSkill>,
    z_ability: ZAbilities,
    is_lf: bool,
    is_tag: bool,
    has_zenkai: bool,
}
```
You can see all the enums and structs used in [struct.rs](https://github.com/feijoes/DBlegendsAPI/blob/main/api/src/types/structs.rs) and in [enums.rs](https://github.com/feijoes/DBlegendsAPI/blob/main/api/src/types/enums.rs)

### URLs
The current URL for the api is `http://localhost:1000/api/v1/`

At methor GET should return a list of all characters 
> Example JSON returned
```json
[
  {
    "name": "Legendary Super Saiyan Broly",
    "id": "DBL51-03U",
    "color": "PUR",
    "rarity": "ULTRA",
    "tags": [
      "Saiyan",
      "Super Saiyan",
      "Powerful Opponent",
      "Male",
      "ULTRA",
      "Melee Type",
      "PUR",
      "Sagas From the Movies",
      "Broly",
      "Rising Power",
      "Armored Strike Arts",
      "Ultimate Arts",
      "Special Cover Change (Unique)",
      "Special Cover Change Nullification",
      "Unique Gauge",
      "No Switching"
    ],
    "main_ability": {
      "name": "What? Finished already?",
      "effect": "Draw the Ultimate Arts Card \"Gigantic Overflow\" next.Restores own health by 20% and Ki by 100.Reduces enemy Ki by 70.Inflicts enemy with Attribute Downgrade \"-100% to Ki Recovery\" for 5 timer counts.Requirements: 25 timer counts must elapse."
    },
    "unique_ability": {
      "unique_start_abilities": [
        {
          "ability_name": "Awakened Instincts",
          "ability_effect": "Applies the following effects to self when battle starts:+110% to damage inflicted (cannot be cancelled).Reduces damage received by 60% (cannot be cancelled).+150% to Ki Recovery (cannot be cancelled).Increases Unique Gauge by 50%.Applies the following effects to self when this character enters the battlefield:Restores health by 10%.Restores Ki by 50.The following effects occur when enemy switches characters while this character is on the battlefield:Nullifies unfavorable Element factors for damage sustained for 5 timer counts.Inflicts all enemies with Attribute Downgrade \"-30% to Critical Rate\" for 10 timer counts.Applies the following effects to self every 3 timer counts while this character is on the battlefield:+10% to damage inflicted (up to 30%).Effect resets after own character switch.-3 to Special Move Arts cost (up to 15).Effect resets after own character switch.Increases Arts Card Draw Speed by 1 level (up to 2).Effect resets after own or enemy character switch."
        },
        {
          "ability_name": "Power Through Rage",
          "ability_effect": "Nullifies \"Abnormal Condition: Immobilize\" (cannot be cancelled).Charges own Unique Gauge every time when hit with an enemy's Arts attack while this character is on the battlefield.Once the Unique Gauge is full, it resets to zero and the following effects occur:Reduces enemy Vanishing Gauge to 0% (activates four times).*Effect will not activate if this character is defeated.Reduces enemy Ki by 30.Randomly destroys 1 enemy card.Reduces enemy's Dragon Balls by 1 (activates once).Restores own health by 15% (activates four times).Applies Buff Effect \"Nullifies enemy's special actions that activate when changing cover\" to self for 10 timer counts.Cancels own Attribute Downgrades.Knocks enemy back to long range if a cover change is performed against the following attacks (activates twice) (activates during assists).Strike ArtsBlast ArtsSpecial Arts (excluding some such as wide-range attacks, etc.)Special Move Arts (excluding some such as wide-range attacks, etc.)Awakened Arts (excluding some)*Cover changes performed against attacks other than those listed are also counted as activations.*Nullified activations are not counted.[Comboable Arts]Special ArtsSpecial Move Arts"
        }
      ]
    },
    "ultra_ability": {
      "name": "Resonance of Force (Sagas From the Movies)",
      "effect": "If this character is the Leader, applies the following effects to self when battle starts:+30% to damage inflicted (cannot be cancelled).+30% to Ki Recovery (cannot be cancelled).If this character is not the Leader, applies the following effects to self per \"Episode: Sagas From the Movies\" battle/support member when battle starts:+5% to damage inflicted (cannot be cancelled).+5% to Ki Recovery (cannot be cancelled).*Up to 3 support members will be counted."
    },
    "base_stats": {
      "power": 10488,
      "health": 26274,
      "strike_atk": 2772,
      "strike_def": 1650,
      "blast_atk": 2623,
      "blast_def": 1633
    },
    "max_stats": {
      "power": 933153,
      "health": 1605029,
      "strike_atk": 169339,
      "strike_def": 100772,
      "blast_atk": 160212,
      "blast_def": 99779
    },
    "strike": "Strike (Impact) with Blast Damage armor.",
    "shot": "Blast (Impact)",
    "image_url": "https://i.imgur.com/xmAgY6X.png",
    "special_move": {
      "name": "Gigantic Bomb",
      "effect": "Deals massive Explode damage.Applies the following effects to self according to the number of remaining enemy battle members upon activation:3 battle members: +50% to Special Move damage inflicted for 3 timer counts.2 battle members: +30% to Special Move damage inflicted for 3 timer counts.1 battle member: +20% to Special Move damage inflicted for 3 timer counts.*Blast Armor when charging forward."
    },
    "special_skill": {
      "name": "Gigantic Volley",
      "effect": "Applies the following effects to self upon activation:Restores Ki by 30.Restores Vanishing Gauge by 70%.The following effects occur on hit:Inflicts all enemies with \"No Switching\" for 5 timer counts.Seals all enemies' Main Abilities for 5 timer counts.Increases own Unique Gauge by 20%.*Blast Armor when charging forward."
    },
    "ultimate_skill": {
      "name": "Gigantic Overflow",
      "effect": "Deals supreme Explode damage.Applies the following effects to self upon activation:+50% to Ultimate damage inflicted for 3 timer counts.Nullifies enemy's \"Restores health when it reaches 0\" effects when this character attacks for 3 timer counts.30% chance to inflict Faint on hit.*Blast Armor when charging forward."
    },
    "z_ability": {
      "one": {
        "tags": ["Sagas From the Movies"],
        "effect": "+28% to \"Episode: Sagas From the Movies\" base Strike & Blast Defense during battle."
      },
      "two": {
        "tags": ["Sagas From the Movies", "Powerful Opponent"],
        "effect": "+30% to \"Episode: Sagas From the Movies\" or \"Tag: Powerful Opponent\" base Strike & Blast Defense during battle."
      },
      "three": {
        "tags": [
          "Sagas From the Movies",
          "Sagas From the Movies",
          "Powerful Opponent"
        ],
        "effect": "+3% to Ultimate & Awakened damage inflicted by \"Episode: Sagas From the Movies\" and +38% to \"Episode: Sagas From the Movies\" or \"Tag: Powerful Opponent\" base Strike & Blast Defense during battle."
      },
      "four": {
        "tags": [
          "Sagas From the Movies",
          "Sagas From the Movies",
          "Powerful Opponent"
        ],
        "effect": "+5% to Ultimate & Awakened damage inflicted by \"Episode: Sagas From the Movies\" and +42% to \"Episode: Sagas From the Movies\" or \"Tag: Powerful Opponent\" base Strike & Blast Defense during battle."
      }
    },
    "is_lf": false,
    "is_tag": false,
    "has_zenkai": false
  }
]
```
### API params
You can filter or sort the character for this categories at `http://localhost:1000/api/v1/?Param=value`

Field | Description | example
------|------------ | ------
name  | The exact name of the character. You can pass multiple options separated with a "," | `/api/v1/?name=Super%20Saiyan%202%20Kefla` is searching for Super Saiyan 2 Kefla
fname | All the characters names that contains the string given, You can pass multiple options separated with a "," | `/api/v1/?fname=Broly` is searching for all characters that contains Broly in their name
has_zenkai | Show only characters with or without zenkai | `/api/v1/?fname=Goku&has_zenkai=false` is searching for all Goku characters that dont have a zenkai
has_ultimate | Show only characters  with or without ultimate card | `/api/v1/?fname=Goku&has_ultimate=true` is searching for all tag Goku characters
is_lf | Show only characters that is or is not a lf character | `/api/v1/?fname=Goku&is_lf=true` is searching for all LF Goku characters
is_tag | Show only characters that is or is not a tag character | `/api/v1/?fname=Goku&is_tag=true` is searching for all tag Goku characters
num | Show a limit n characters | `/api/v1/?fname=Goku&num=2` show only 2 characters with Goku in their name 
id | The exact id of the character, You can pass multiple options separated with a "," | `/api/v1/?id=DBL09-06S` is searching for the character with id DBL09-06S
color | All the characters that have the specify color, You can pass multiple options separated with a "," |  `/api/v1/?color=PUR,GRN` is searching for all the characters with color GRN or PUR
rarity | All the characters that have the specify rarity,, You can pass multiple options separated with a "," | `/api/v1/?rarity=HERO` is searching for all Hero characters 
tags | Show all the characters with the specifys categories, You can pass multiple tags separated with a ","| `/api/v1/?categories=Super%20Saiyan,Powerful%20Opponent` is searching for all characters that has the tags Super Saiyan y Powerfull Opponent
sort_by | Sort the characters for a specify order | `/api/v1/?sort_by=color` show all the characters sorted by highest Max Level Attack
reverse | only if sort_by is used, show in reverse order | `/api/v1/?sort_by=Name&reverse=true` show all the characters sorted by Name in reverse 'z-a'


### Current options
Field | Options 
------|------------ |
rarity | HERO,, EXTREME, SPARKING, ULTRA
sort_by | Name, Rarity, Color
color | PUR, YEL, BLU, RED, GRN, LGT
