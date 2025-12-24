```md
# CI/CD & Releases

Aargal uses GitHub Actions.

---

## Workflows

````

.github/workflows/
├── build.yml
└── release.yml

````

---

## CI Flow

```mermaid
flowchart TD
    A[Push / PR] --> B[CI]
    B --> C[Build + Test]
    C --> D{Pass?}
    D -- No --> E[Fail]
    D -- Yes --> F[Success]
````

---

## Release Flow

```mermaid
flowchart TD
    A[Tag vX.Y.Z] --> B[Release Workflow]
    B --> C[Build Binary]
    C --> D[Create Release]
    D --> E[Attach Assets]
```

---

## Releasing

```bash
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

---

## Result

* GitHub Release created
* Binary assets attached
* Ready for install.sh