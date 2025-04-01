## Chad-bot rústico
Bot de la comunidad de [Discord](https://discord.gg/FSKeeDhMNN).

## Desarrollo

### Variables de entorno
> Antes de compilar el proyecto necesitas definir algunas variables de entorno para que compile correctamente
> Definir las variables de entorno en un archivo llamado: `Secrets.toml` basadas en el archivo: [`./Secrets.toml.example`](./Secrets.toml.example)

### Para ejecutar el código en `develop mode`:

Se debe de instalar `shuttle` para ello usar `cargo-binstall`.

#### 🍎 Mac / 🐧Linux:
```bash
curl -sSfL https://www.shuttle.dev/install | bash
```

####  Para Windows:

```powershell
iwr https://www.shuttle.dev/install-win | iex
```

Una vez instalado instalamos shuttle:

```
cargo binstall cargo-shuttle
```


### Ejecutar el bot:

Luego ejecuta el siguiente comando para ejecutar de modo local el bot:
```bash
shuttle run
```

## Producción

Para ejecutar el bot en modo producción debemos ejecutar el siguiente comando:

```bash
shuttle deploy
```

Esto deployara en Shuttle el bot.

> Documentación de [Shuttle](https://docs.shuttle.rs/getting-started/installation) para más información.
