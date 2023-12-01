# FT_TP110
Contrôle de la prise Tapo TP110 avec Jeedom

## Introduction

Lorsque je me suis retrouvé dans l'incertitude quant à la possibilité d'utiliser ma prise avec Jeedom, j'ai décidé d'installer Home Assistant pour vérifier si la prise était compatible. À ma grande joie, elle fonctionnait parfaitement. Intrigué, j'ai entrepris de découvrir comment cela était possible et j'ai découvert un dépôt GitHub très utile.

Le dépôt en question : [https://github.com/mihai-dinculescu/tapo](https://github.com/mihai-dinculescu/tapo)

L'auteur a créé une API en utilisant le langage Rust (également appelable en Python). Grâce à cela, j'ai réussi à faire fonctionner ma prise avec Rust. Idéalement, j'aurais aimé effectuer un portage en PHP, mais cela s'est avéré complexe. Après avoir consacré plusieurs soirées à cette tâche et confronté à des problèmes de hachage différents entre Rust et PHP (ma connaissance de Rust étant limitée), j'envisage d'abandonner cette solution de portage en raison de sa complexité, en l'absence de documentation de la part de TP LINK et des défis potentiels de maintenance en cas de changement ou de mise à jour.

D'après mes investigations, à partir d'une certaine version du firmware, TP LINK a renforcé la sécurité de la prise en introduisant une authentification à deux étapes. La première étape consiste à envoyer une clé à la prise, que le programme doit utiliser pour chiffrer le nom d'utilisateur et le mot de passe lors de la deuxième étape, tout en obtenant des informations de session.

En attendant une éventuelle mise à jour du plugin wifilightv2 ou d'un autre plugin compatible, j'envisage d'utiliser cette API Rust via le plugin Jeedom SCRIPT.

Discussion sur le problème sur le forum JEEDOM : [P110 ne fonctionne plus](https://community.jeedom.com/t/p110-ne-fonctionne-plus/116182)

Discussion sur le plugin officiel WifiLightV2 : [Plugin WifiLightV2](https://community.jeedom.com/tag/plugin-wifilightv2)

Lien vers la documentation du plugin WifiLightV2 : [Documentation WifiLightV2](https://bcaro.github.io/wifilightV2-doc/fr_FR/)

Lien vers la documentation du plugin Script Jeedom : [Documentation Script Jeedom](https://doc.jeedom.com/fr_FR/plugins/programming/script/beta/)

Crédit à Mihai Dinculescu pour son travail remarquable.

## Installation

Voici le cheminement pour réussir à installer cela au sein de Jeedom et obtenir ce genre de virtuel :

![Exemple-Objet-Jeedom-Virtuel](https://github.com/fran6t/FT_TP110/assets/4406087/7d2df66f-f5c2-4134-b314-b2571cc10378)

Dans le wiki sont indiquées :
- l'installation de Rust, la compilation, le fichier de configuration,
- les procédures de configuration de l'objet Script, d'un objet Virtuel et de l'objet MQTT.
