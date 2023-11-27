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


Obtenir l'Exécutable pour le Dialogue avec la ou les Prises

Connectez-vous à la machine Jeedom avec le terminal.
Clonage du Dépôt

Pour obtenir une copie du projet sur votre machine, exécutez les commandes suivantes dans votre console SSH :

```bash
git clone https://github.com/fran6t/FT_TP110.git
cd FT_TP110
```

Note : Nous avons besoin du langage Rust. Vérifiez s'il est déjà installé sur la machine en tapant la commande suivante :

```bash
cargo -V
```

Si vous obtenez une version, cela signifie que Rust est déjà installé. Par exemple, chez moi, je ne me souvenais pas l'avoir installé pour gérer un purificateur d'air Xiaomi.

Si Rust est installé, vous pouvez le mettre à jour en utilisant la commande :

```bash
rustup update
```

Si Rust n'est pas installé sur la machine, vous pouvez le faire avec la commande suivante :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

La désinstallation de Rust, si nécessaire, se fait en tapant :

```bash
rustup self uninstall
```

Maintenant, nous sommes prêts à effectuer un premier test avec notre prise.

À l'aide de l'application TAPO, obtenez votre login, mot de passe et l'adresse IP de la prise.

Effectuez un premier test en lançant la commande suivante :

```bash
cargo run
```

Cela va construire l'application puis lancer le programme mais comme nous n'avons pas donné de paramètres nous obtenons 

```bash
Finished dev [unoptimized + debuginfo] target(s) in 0.16s
Running `target/debug/ft_tp110`
Usage: cargo run <adresse_ip> <login> <pass> <action>
```

Maintenant avec les bon parametre comme cela nous voyons qu'il y a 1 en reponse e, effet en passant on en parametre le programme repond 1
Si nous passons off il doit y avoir 0 en réponse
Si nous passons get_current_power il doit y avoir 0 si la prise et eteint ou sans charge et sinon un chiffre qui reflete la puissance en W qui passe au travers de la prise

Pour mémoire l'adresse mail et le mot de passe sont les mêmes qui ont servis à la configuration et/ou l'utilisation de la prise avec l'application TAPO 

```bash
cargo run 192.168.0.70 monadressemail monmotdepasse on
Finished dev [unoptimized + debuginfo] target(s) in 0.16s
Running `target/debug/ft_tp110 192.168.0.70 'monadressemail' monmotdepasse on`
1

```

Si vous arrivez a commander votre prise, nous pouvons alors passer à la génération du binaire qui servira a Jeedom 
pour cela nous lançons la commande ci-dessous 

```bash
cargo build --release
```

Nous obtenons donc un binaire que nous pouvons maintenant copier au sein de Jeedom 

```bash
cp target/release/ft_tp110 /var/www/html/plugins/script/data/
```

On change le propriétaire pour que Jeedom puisse avoir la main dessus
```bash
chown www-data:www-data /var/www/html/plugins/script/data/ft_tp110


Maintenant le reste se passe dans Jeedom avec le plugins script


Etape 0

On installe le plugin script officiel Jeedom

Etape 1

Etape 2

Etape 3

Etape 4

Etape 5

On recommemce avec la commande off

Etape 6

On crée une commande info pour avoir le retour de la puissance qui traverse la prise








