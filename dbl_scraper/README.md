# Project

This is a scraper for [Dragon Ball Legends Space](https://legends.dbz.space/) characters page

## Prerequisites

Make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Getting Started

1. Clone the repository:

   ```bash
   git clone https://github.com/feijoes/DBlegendsAPI.git
   ```

2. cd to scraper 

    ```bash
    cd  ./DBlegendsAPI/dbl_scraper
    ```
3. run the project
    ```bash
    cargo run
    ```

This will create a json file in "data/Characters_{currect date}.json" with the info of all characters available

Character Json
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
      "ability1_name": "Awakened Instincts",
      "ability1_effect": "Applies the following effects to self when battle starts:+110% to damage inflicted (cannot be cancelled).Reduces damage received by 60% (cannot be cancelled).+150% to Ki Recovery (cannot be cancelled).Increases Unique Gauge by 50%.Applies the following effects to self when this character enters the battlefield:Restores health by 10%.Restores Ki by 50.The following effects occur when enemy switches characters while this character is on the battlefield:Nullifies unfavorable Element factors for damage sustained for 5 timer counts.Inflicts all enemies with Attribute Downgrade \"-30% to Critical Rate\" for 10 timer counts.Applies the following effects to self every 3 timer counts while this character is on the battlefield:+10% to damage inflicted (up to 30%).Effect resets after own character switch.-3 to Special Move Arts cost (up to 15).Effect resets after own character switch.Increases Arts Card Draw Speed by 1 level (up to 2).Effect resets after own or enemy character switch.",
      "ability2_name": "Power Through Rage",
      "ability2_effect": "Nullifies \"Abnormal Condition: Immobilize\" (cannot be cancelled).Charges own Unique Gauge every time when hit with an enemy's Arts attack while this character is on the battlefield.Once the Unique Gauge is full, it resets to zero and the following effects occur:Reduces enemy Vanishing Gauge to 0% (activates four times).*Effect will not activate if this character is defeated.Reduces enemy Ki by 30.Randomly destroys 1 enemy card.Reduces enemy's Dragon Balls by 1 (activates once).Restores own health by 15% (activates four times).Applies Buff Effect \"Nullifies enemy's special actions that activate when changing cover\" to self for 10 timer counts.Cancels own Attribute Downgrades.Knocks enemy back to long range if a cover change is performed against the following attacks (activates twice) (activates during assists).Strike ArtsBlast ArtsSpecial Arts (excluding some such as wide-range attacks, etc.)Special Move Arts (excluding some such as wide-range attacks, etc.)Awakened Arts (excluding some)*Cover changes performed against attacks other than those listed are also counted as activations.*Nullified activations are not counted.[Comboable Arts]Special ArtsSpecial Move Arts"
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
