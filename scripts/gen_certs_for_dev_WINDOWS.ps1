# Gen Certs for Dev - PowerShell Version

# Create certs directory
$certsDir = "..\target\debug\certs"
New-Item -ItemType Directory -Force -Path $certsDir | Out-Null

# Generate self-signed certificate
$cert = New-SelfSignedCertificate `
    -Subject "CN=localhost" `
    -KeyAlgorithm RSA `
    -KeyLength 2048 `
    -CertStoreLocation "Cert:\CurrentUser\My" `
    -NotAfter (Get-Date).AddDays(365)

# Export certificate and private key
Export-PfxCertificate `
    -Cert $cert `
    -FilePath "$certsDir\cert.pfx" `
    -Password (ConvertTo-SecureString -String "password" -Force -AsPlainText) | Out-Null

# Export just the public key
Export-Certificate `
    -Cert $cert `
    -FilePath "$certsDir\cert.pem" -Type CERT | Out-Null

# Clean up - remove from certificate store
Remove-Item -Path $cert.PSPath

Write-Host "Self-signed certificates generated in $certsDir"
Write-Host "Note: The PFX file contains both certificate and private key (password: 'password')"