# Quiz GUI (Rust + egui/eframe)

Application GUI pour passer des quiz interactifs avec support d'images, développée en Rust avec egui/eframe.

## Prérequis
- Rust (toolchain stable, édition 2024)
- macOS (testé), mais compatible Linux/Windows

## Structure du projet

```
quiz-gui/
├── Cargo.toml              # Configuration du projet Rust
├── Config.yaml             # Configuration de l'application
├── README.md
├── icon.png                # Icône de l'application
├── fonts/                  # Polices personnalisées
│   ├── MPLUSRounded1c-Regular.ttf
│   └── NotoSansSymbols2-Regular.ttf
├── Quizzes/                # Dossier des quiz (ignoré par Git)
│   ├── base.txt            # Exemple de quiz simple
│   ├── example_with_images.txt  # Exemple avec images
│   └── images/             # Images pour les questions
│       ├── banane.png
│       ├── fraise.png
│       ├── orange.png
│       └── pomme.png
└── src/                    # Code source
    ├── main.rs            # Point d'entrée
    ├── app.rs             # Logique de l'interface utilisateur
    ├── config.rs          # Chargement de Config.yaml
    ├── fonts.rs           # Gestion des polices
    ├── models.rs          # Structures de données (Quiz, Answer)
    └── quiz_loader.rs     # Chargement des fichiers quiz
```

## Configuration (Config.yaml)

```yaml
app_title: "Quiz GUI"
quiz_path: "Quizzes"
window_width: 600.0
window_height: 400.0
```

## Format des quiz

Les quiz sont des fichiers `.txt` au format YAML dans le dossier `Quizzes/` :

```yaml
questions:
  - question: "Quelle est la capitale de la France ?"
    answers:
      - letter: A
        text: "Londres"
      - letter: B
        text: "Paris"
      - letter: C
        text: "Berlin"
      - letter: D
        text: "Madrid"
    correct_answer: B
```

### Support des images

Vous pouvez ajouter des images aux réponses :

```yaml
questions:
  - question: "Lequel de ces fruits est la banane ?"
    answers:
      - letter: A
        text: pomme
        image: "Quizzes/images/pomme.png"
      - letter: B
        text: banane
        image: "Quizzes/images/banane.png"
    correct_answer: B
```

## Lancer l'application

```zsh
cargo run
```

L'application charge automatiquement :
- Les quiz depuis le dossier `Quizzes/`
- La configuration depuis `Config.yaml`
- Les polices personnalisées depuis `fonts/`
- L'icône depuis `icon.png`

## Fonctionnalités

- **Sélection de quiz** : Si plusieurs quiz sont disponibles, un écran de sélection s'affiche
- **Navigation** : Boutons Précédent/Suivant pour parcourir les questions
- **Sélection de réponse** : Interface avec boutons radio pour chaque option
- **Support d'images** : Affichage d'images dans les réponses avec mise en cache
- **Résultats** : Affichage du score final avec détail des réponses correctes/incorrectes
- **Police personnalisée** : Support de caractères spéciaux et emojis
- **Interface responsive** : Adaptation automatique à la taille de la fenêtre

## Dépendances principales

- `eframe` 0.33.2 - Framework d'application
- `egui` 0.33.2 - Bibliothèque GUI immédiate
- `egui_extras` - Support du chargement d'images
- `serde` & `serde_yaml` - Parsing des fichiers de configuration et quiz
- `image` - Traitement des images

## Notes de développement

- Le dossier `Quizzes/` est ignoré par Git (`.gitignore`) pour ne pas commiter les quiz personnels
- Les polices TTF permettent un meilleur rendu des caractères Unicode
- Le cache d'images évite de recharger les mêmes fichiers plusieurs fois
- L'édition 2024 de Rust est utilisée pour bénéficier des dernières fonctionnalités

## Licenses

### Code source
Ce projet est distribué sous double licence :
- **MIT License** - Voir [LICENSE-MIT](LICENSE-MIT)
- **Apache License 2.0** - Voir [LICENSE-APACHE](LICENSE-APACHE)

Vous pouvez choisir la licence qui convient le mieux à votre usage.

### Polices
Les polices incluses dans le dossier `fonts/` sont sous licence **SIL Open Font License (OFL)** - Voir [fonts/OFL.txt](fonts/OFL.txt)
