Phone Number Manager

Dépôt du projet :  
https://github.com/alexgd69/Phone_Number_Manager

Contributors :

Alexis Brun : alexgd69
Axel Laïb : codeaxel69
Moussa Kone : btlsharif
Rayane Derrag 

Description

Projet Rust permettant de stocker des numéros de téléphone dans une structure de données appelée **Trie** (*prefix tree*), puis de générer une visualisation sous forme de **MindMap PlantUML**.

Objectif

Le programme :

1. lit un fichier JSON contenant des contacts ;
2. construit un Trie en mémoire ;
3. insère les numéros de téléphone ;
4. génère un fichier `.puml` ;
5. permet de visualiser la structure sous forme de graphe.

Prérequis

Avant de lancer le projet, vous devez installer :

- Rust : https://www.rust-lang.org/tools/install
- Cargo (installé automatiquement avec Rust)

Vérifier l’installation :

```bash
rustc --version
cargo --version


Cloner le dépôt:

git clone https://github.com/alexgd69/Phone_Number_Manager.git
cd Phone_Number_Manager

Récupérer les mises à jour:

git pull

Lancer le projet :

cargo run

Ajouter vos modifications:

git add .
git commit -m "feat: update project"
git push