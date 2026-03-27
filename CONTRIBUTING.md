# Contributing to Archflow

Thank you for your interest in Archflow.
This project is still early, so clear communication and small, well-scoped contributions are especially appreciated.

---

## English

### Ways to contribute

You can contribute in several ways:

- report bugs
- propose features
- suggest architecture rules
- improve docs
- improve examples
- implement scoped issues

### Before opening an issue

Please check whether the same topic already exists.
If not, choose the most appropriate issue type:

- **Bug report**: something is broken or inconsistent
- **Feature request**: a new capability or improvement
- **Architecture rule request**: a new rule, contract pattern, placement rule, or artifact policy

### Before opening a pull request

Please keep pull requests focused.
For large changes, open an issue first so we can align on scope and direction.

### Contribution principles

When contributing, please keep these principles in mind:

1. **Artifact-first**
   Prefer explicit artifact contracts over implicit assumptions.

2. **Sidecar-first**
   Prefer language-agnostic metadata such as YAML or Markdown when possible.

3. **Design before automation**
   Avoid adding smart automation before the contract model is clear.

4. **Small and explainable**
   Favor simple behavior that is easy to understand and verify.

5. **AI-friendly but human-readable**
   Outputs should work well for AI tools, but must remain understandable to humans.

### Pull request checklist

Before submitting a PR, please confirm:

- the change is scoped and explained
- documentation is updated if needed
- naming is consistent
- the intent is clear for both humans and AI users
- examples are updated when behavior changes

### Commit and PR style

Recommended style:

- `docs: improve README wording`
- `feat: add scaffold command skeleton`
- `fix: correct contract file path resolution`
- `design: revise artifact contract terminology`

### Good first contributions

Good early contributions include:

- README refinements
- terminology clarification
- contract schema suggestions
- example project improvements
- issue template improvements

---

## 日本語

### 貢献のしかた

Archflow には、次のような形で貢献できます。

- バグ報告
- 機能提案
- アーキテクチャルール提案
- ドキュメント改善
- サンプル改善
- 小さく分割された issue の実装

### Issue を作る前に

同じ内容がすでに登録されていないか確認してください。
そのうえで、適切な issue 種別を選んでください。

- **Bug report**: 動作不良や不整合
- **Feature request**: 新機能や改善提案
- **Architecture rule request**: 配置ルール、contract パターン、artifact 方針の提案

### Pull Request を作る前に

PR はできるだけ小さく、目的が明確なものにしてください。
大きな変更は、先に issue で方向性をそろえてから進めてください。

### 貢献時の原則

1. **Artifact-first**
   暗黙知ではなく、artifact 単位の契約を重視します。

2. **Sidecar-first**
   可能な限り、言語非依存な YAML / Markdown などのメタ情報を優先します。

3. **Design before automation**
   contract モデルが固まる前に、過度な自動化を入れないでください。

4. **Small and explainable**
   複雑すぎる仕組みより、説明しやすく検証しやすい仕組みを優先します。

5. **AI-friendly but human-readable**
   AI が使いやすいだけでなく、人間にも読みやすいことを大切にします。

### PR チェックリスト

PR を送る前に、次を確認してください。

- 変更範囲が明確か
- 必要なドキュメント更新があるか
- 命名が一貫しているか
- 人間にも AI にも意図が伝わるか
- 振る舞いが変わるなら examples も更新されているか

### コミット / PR のスタイル例

- `docs: improve README wording`
- `feat: add scaffold command skeleton`
- `fix: correct contract file path resolution`
- `design: revise artifact contract terminology`

### 最初の貢献としておすすめ

- README の改善
- 用語整理
- contract schema の提案
- example project の改善
- issue template の改善

---

## Communication / コミュニケーション

Please be respectful, specific, and constructive.

敬意を持ち、具体的で、建設的なコミュニケーションをお願いします。
