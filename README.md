\# Phone Number Manager



Gestionnaire de numéros de téléphone basé sur un Trie (prefix tree), implémenté en Rust.



\## Lancer le projet



```bash

cargo run --release

```



Le programme lit les données dans `data/04\_common\_parts.json` et génère le fichier PlantUML dans `graph/04\_common\_parts.puml`.



\## Visualiser le graphe avec PlantUML



1\. Installer Docker : https://docs.docker.com/desktop/

2\. Lancer le serveur PlantUML :

```bash

docker pull plantuml/plantuml-server:jetty

docker run -d -p 8080:8080 plantuml/plantuml-server:jetty

```

3\. Ouvrir http://localhost:8080/ dans un navigateur

4\. Copier-coller le contenu de `graph/04\_common\_parts.puml`

test axel

