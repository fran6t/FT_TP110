<?php
// Essai de fonctionnement de la prise tp110 via php 
// Projet abandonné
class KlapCipher {
    // ... (les autres fonctions de la classe restent inchangées)

    public static function sha256($data) {
        return hash('sha256', $data, true);
    }

    public static function deriveIv($localHash) {
        $localHash = array_merge(['iv'], str_split($localHash));
        $hash = self::sha256(implode('', $localHash));
        $iv = substr($hash, 0, 12);
        $seq = unpack('N', substr($hash, -4));
        return ['iv' => $iv, 'seq' => $seq[1]];
    }
}

$yourUsername = "francis.trautmann@gmil.com";
$yourPassword = "Zsj2bJBMKHT";

// Générer le local_seed
$localSeed = random_bytes(16);

// Calculer le auth_hash
//$authHash = hash('sha256', hash('sha1', $yourUsername) . hash('sha1', $yourPassword), true);
//$authHash = hash('sha256', hash('sha1', $yourUsername, true) . hash('sha1', $yourPassword, true), true);
$authHash = hash('sha256', hash('sha1', $yourUsername, true) . hash('sha1', $yourPassword, true) . $localSeed, true);


// Dérivation de l'IV
$ivInfo = KlapCipher::deriveIv($localSeed);
$iv = $ivInfo['iv'];
$seq = $ivInfo['seq'];

var_dump("Local Seed: " . bin2hex($localSeed));
var_dump("Auth Hash: " . bin2hex($authHash));
var_dump("IV: " . bin2hex($iv));
var_dump("Seq: " . $seq);

// Construire la requête POST
$data = $localSeed;
$ch = curl_init('http://192.168.0.70/app/handshake1');
curl_setopt($ch, CURLOPT_POST, 1);
curl_setopt($ch, CURLOPT_POSTFIELDS, $data);
curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);

// Exécuter la requête
$response = curl_exec($ch);

// Vérifier la réponse
$httpCode = curl_getinfo($ch, CURLINFO_HTTP_CODE);
curl_close($ch);

if ($httpCode == 200) {
    echo "Hex Response: " . bin2hex($response) . "\n";
    // La prise répond, traiter la réponse si nécessaire
    $remoteSeed = substr($response, 0, 16);
    $serverHash = substr($response, 16, 32);

    // Vérifier le hash du serveur
    $localHash = hash('sha256', $localSeed . $remoteSeed . $authHash, true);

    echo "Local Hash: " . bin2hex($localHash) . "\n";
    echo "Server Hash: " . bin2hex($serverHash) . "\n";

    if ($localHash !== $serverHash) {
        die("Local hash does not match server hash");
    }

    echo "Handshake 1 OK\n";
} else {
    // La prise est injoignable
    echo "La prise est injoignable.\n";
}
?>
