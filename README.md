# FT_TP110
Contrôle de la prise Tapo TP110 avec Jeedom
Introduction

Lorsque je me suis retrouvé dans l'incertitude quant à la possibilité d'utiliser ma prise avec Jeedom, j'ai décidé d'installer Home Assistant pour vérifier si la prise était compatible. À ma grande joie, elle fonctionnait parfaitement. Intrigué, j'ai entrepris de découvrir comment cela était possible et j'ai découvert un dépôt GitHub très utile.

Le dépôt en question : mihai-dinculescu/tapo

L'auteur a créé une API en utilisant le langage Rust (également appelable en Python). Grâce à cela, j'ai réussi à faire fonctionner ma prise avec Rust. Idéalement, j'aurais aimé effectuer un portage en PHP, mais cela s'est avéré complexe. Après avoir consacré plusieurs soirées à cette tâche et confronté à des problèmes de hachage différents entre Rust et PHP (ma connaissance de Rust étant limitée), j'envisage d'abandonner cette solution de portage en raison de sa complexité, en l'absence de documentation de la part de TP LINK et des défis potentiels de maintenance en cas de changement ou de mise à jour.

D'après mes investigations, à partir d'une certaine version du firmware, TP LINK a renforcé la sécurité de la prise en introduisant une authentification à deux étapes. La première étape consiste à envoyer une clé à la prise, que le programme doit utiliser pour chiffrer le nom d'utilisateur et le mot de passe lors de la deuxième étape, tout en obtenant des informations de session.

En attendant une éventuelle mise à jour du plugin wifilightv2 ou d'un autre plugin compatible, j'envisage d'utiliser cette API Rust via le plugin Jeedom SCRIPT.

Discussion sur le probleme forum JEEDOM https://community.jeedom.com/t/p110-ne-fonctionne-plus/116182
Discussion plugin officiel WifiLightV2 https://community.jeedom.com/tag/plugin-wifilightv2
Lien doc plugin WifiLightV2 https://bcaro.github.io/wifilightV2-doc/fr_FR/
Lien doc plugin script https://doc.jeedom.com/fr_FR/plugins/programming/script/beta/


Pour obtenir l'executable qui va permettre le dialogue avec la ou les prises :

Connectez vous sur la machine Jeedom avec le terminal

## Clonage du Dépôt

Pour obtenir une copie du projet sur votre machine, exécutez les commandes suivantes dans votre console SSH :

```bash
git clone https://github.com/fran6t/FT_TP110.git
cd FT_TP110
```

Nous avons besoins du langage Rust

Pour cela essayons de voir s'il est pas déjà sur la machine en tapant 

```bash
cargo -V 
```

Si vous obtenez une version c'est que Rust est déjà là par exemple chez moi je me souvenez plus l'avoir installer pour gerer un purificateur d'air xiomi

Si RUST est intallé vous pouvez le mettre çà jour en faisant

```bash
rustup update
```

Si RUST n'est pas sur la machine vous pouvez l'installer avec la commande 

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

La desinstallation de RUST si besoins se fait en tapant 
```bash
rustup self uninstall
```

Voila nous sommes prêt à faire un premier test de notre prise

Par l'application TAPO, nous connaissons notre login pass et nous pouvons aller voir l'adresse IP de la prise

Nous allons pouvoir effectuer un premier test en lançant la commande 

```bash
cargo run 192.168.0.70 monadressemail monmotdepasse on
```

Cela doit construire l'application puis appeler la prise a l'aide de son adresse ip des infos de login et on pour allumer la prise








