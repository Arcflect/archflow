# Security Policy / セキュリティポリシー

## English

Thank you for helping make Archflow safer.

### Supported versions

Because Archflow is in an early stage, security fixes will generally be applied to the latest development line first.

| Version | Supported |
| --- | --- |
| latest | ✅ |
| older releases | ❌ |

### Reporting a vulnerability

Please **do not open a public issue** for suspected security vulnerabilities.

Instead, report the issue privately to the maintainers through one of the following channels:

- GitHub Security Advisories / private vulnerability reporting, if enabled
- A private contact address listed in the repository profile or organization profile

When reporting, please include as much of the following as possible:

- affected version or commit
- reproduction steps
- impact description
- proof of concept, logs, or screenshots if available
- any suggested mitigation or fix

### What to expect

Maintainers will try to:

- acknowledge receipt within a reasonable time
- confirm whether the issue is a valid security problem
- assess severity and scope
- prepare a fix before public disclosure when appropriate
- credit the reporter when desired

Please note that response times may vary depending on maintainer availability.

### Scope

This policy primarily covers:

- vulnerabilities in Archflow source code
- insecure default behavior introduced by Archflow
- sensitive information exposure caused by project code or official templates

This policy generally does **not** cover:

- issues in third-party services or dependencies unless Archflow specifically introduces the risk
- local misconfiguration in downstream projects
- feature requests framed as security issues without a concrete vulnerability

### Disclosure

Please allow maintainers reasonable time to investigate and prepare a fix before public disclosure.

Responsible disclosure helps protect users and downstream adopters.

---

## 日本語

Archflow をより安全にするためのご協力ありがとうございます。

### サポート対象バージョン

Archflow はまだ初期段階のため、セキュリティ修正は基本的に **最新の開発系統** を優先して対応します。

| バージョン | サポート |
| --- | --- |
| latest | ✅ |
| それ以前のリリース | ❌ |

### 脆弱性の報告方法

セキュリティ上の問題が疑われる場合は、**公開 Issue を作成しないでください**。

代わりに、以下のいずれかの方法で非公開にてご連絡ください。

- GitHub Security Advisories / 非公開の脆弱性報告機能（有効化している場合）
- リポジトリ所有者または組織プロフィールに記載された非公開連絡先

報告時には、可能な範囲で次の情報を含めてください。

- 影響を受けるバージョンまたはコミット
- 再現手順
- 想定される影響
- 可能であれば PoC、ログ、スクリーンショット
- 考えられる暫定対処や修正案

### 報告後の対応

メンテナーは、可能な限り以下を目指します。

- 妥当な期間内に受領を通知する
- セキュリティ問題として有効か確認する
- 影響範囲と深刻度を評価する
- 必要に応じて公開前に修正を準備する
- 希望があれば報告者へ謝辞を記載する

ただし、対応速度はメンテナーの稼働状況により変わる場合があります。

### 対象範囲

主に以下を対象とします。

- Archflow 本体のソースコードに含まれる脆弱性
- Archflow が導入する危険なデフォルト動作
- 公式テンプレートやプロジェクトコードによる機微情報の漏えい

原則として、以下は対象外です。

- Archflow 自体が直接原因ではない第三者サービスや依存ライブラリの問題
- 利用側プロジェクト固有のローカル設定ミス
- 具体的な脆弱性を伴わない機能要望

### 公開について

脆弱性の公開は、メンテナーが調査し修正準備を行うための妥当な時間を確保した上でお願いします。

責任ある開示は、利用者と downstream project を守ることにつながります。
