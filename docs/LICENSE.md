# License Information

## MIT License

Swoop is released under the **MIT License**, one of the most permissive and widely-used open source licenses.

### Full License Text

```
MIT License

Copyright (c) 2024 Swoop Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## What This Means

### ✅ You CAN:

- **Use commercially**: Use Swoop in commercial products and services
- **Modify**: Make changes to the source code
- **Distribute**: Share Swoop with others
- **Sublicense**: Include Swoop in projects with different licenses
- **Private use**: Use Swoop internally without sharing changes
- **Patent use**: License includes patent rights from contributors

### ❗ You MUST:

- **Include copyright**: Keep the license notice in substantial portions
- **Include license text**: Include the full MIT license text
- **Give credit**: Acknowledge Swoop in your documentation (recommended)

### ❌ You CANNOT:

- **Hold liable**: We're not responsible for damages
- **Expect warranty**: No guarantees about functionality or fitness
- **Use trademarks**: The Swoop name and logo are not licensed

## Comparison with Other Licenses

| Feature | MIT | Apache 2.0 | GPL v3 | BSD 3-Clause |
|---------|-----|------------|--------|--------------|
| Commercial use | ✅ | ✅ | ✅ | ✅ |
| Modification | ✅ | ✅ | ✅ | ✅ |
| Distribution | ✅ | ✅ | ✅ | ✅ |
| Patent protection | ✅ | ✅ | ✅ | ❌ |
| Trademark protection | ❌ | ✅ | ❌ | ❌ |
| Copyleft (viral) | ❌ | ❌ | ✅ | ❌ |
| Length | Short | Long | Very Long | Short |

## Third-Party Dependencies

Swoop includes several third-party libraries with their own licenses:

### Backend Dependencies (Rust)

| Crate | License | Purpose |
|-------|---------|---------|
| `tokio` | MIT | Async runtime |
| `axum` | MIT | Web framework |
| `serde` | MIT/Apache-2.0 | Serialization |
| `sqlx` | MIT/Apache-2.0 | Database driver |
| `reqwest` | MIT/Apache-2.0 | HTTP client |
| `clap` | MIT/Apache-2.0 | CLI parsing |
| `tracing` | MIT | Logging and tracing |
| `uuid` | MIT/Apache-2.0 | UUID generation |
| `chrono` | MIT/Apache-2.0 | Date/time handling |
| `argon2` | MIT/Apache-2.0 | Password hashing |

### Frontend Dependencies (JavaScript/TypeScript)

| Package | License | Purpose |
|---------|---------|---------|
| `react` | MIT | UI framework |
| `next` | MIT | React framework |
| `typescript` | Apache-2.0 | Type system |
| `tailwindcss` | MIT | CSS framework |
| `@tanstack/react-query` | MIT | Server state |
| `axios` | MIT | HTTP client |
| `framer-motion` | MIT | Animations |
| `lucide-react` | ISC | Icons |
| `fumadocs-ui` | MIT | Documentation UI |

### AI and External Services

| Service | License/Terms | Purpose |
|---------|---------------|---------|
| OpenRouter | Commercial Terms | AI model access |
| ElevenLabs | Commercial Terms | Text-to-speech |
| Qdrant | Apache-2.0 | Vector database |

## License Compatibility

### Using Swoop in Your Projects

**✅ Compatible Licenses:**
- MIT, BSD, Apache 2.0: Direct compatibility
- Proprietary/Commercial: Full compatibility
- ISC, Zlib: Compatible permissive licenses

**❓ Special Consideration:**
- GPL v2/v3: Compatible but may affect your project's license
- LGPL: Compatible for linking/dependencies
- Creative Commons: Depends on specific variant

**❌ Incompatible:**
- No known incompatible licenses for typical usage

### Enterprise Considerations

#### Commercial Use
- **No restrictions**: Use Swoop in commercial products freely
- **No royalties**: No payment required for commercial use
- **No attribution required**: Though attribution is appreciated
- **Modify freely**: Customize Swoop for your needs

#### Distribution
- **Source code**: Not required to share your modifications
- **Binary distribution**: Include license notice
- **SaaS/Cloud**: No special requirements for hosted services
- **Mobile apps**: Compatible with app store terms

#### Corporate Policies
Many companies prefer MIT license because:
- **Legal clarity**: Well-understood terms
- **Business friendly**: No copyleft restrictions
- **Patent grant**: Includes basic patent protection
- **Short text**: Easy to review and approve

## Contributor License Agreement

### Individual Contributors

By contributing to Swoop, you agree that:

1. **You own the rights** to your contribution
2. **You grant MIT license** to your contributions
3. **You have authority** to make the contribution
4. **You understand** the contribution is public

### Corporate Contributors

Organizations contributing to Swoop should:

1. **Ensure employees** have authority to contribute
2. **Review IP policies** for open source contributions
3. **Consider signing** a Corporate Contributor License Agreement
4. **Document approval** for significant contributions

## Frequently Asked Questions

### Can I use Swoop in my commercial product?

**Yes!** The MIT license explicitly allows commercial use without restrictions or royalties.

### Do I need to open source my modifications?

**No.** You can keep your modifications private. The MIT license is not "copyleft."

### Can I rebrand Swoop and sell it?

**Yes,** you can modify and sell Swoop, but you must include the original license notice. However, you cannot use the "Swoop" trademark without permission.

### What about patent rights?

The MIT license includes an implicit patent grant from contributors. If you contribute code, you grant patent rights for that contribution.

### Can I contribute to Swoop if I work for a company?

**Usually yes,** but check your employment agreement. Many companies have policies about open source contributions.

### What if I find a license violation?

Please report license violations to legal@swoop.dev. We take license compliance seriously.

## Attribution Guidelines

While not required, attribution helps the open source community:

### Recommended Attribution

**In documentation:**
```
This product includes Swoop (https://github.com/your-org/swoop),
which is licensed under the MIT License.
```

**In software:**
```
// This application uses Swoop - AI-Powered Document Intelligence
// https://github.com/your-org/swoop
// Licensed under MIT License
```

**In about pages:**
```html
<p>
  Powered by <a href="https://github.com/your-org/swoop">Swoop</a> - 
  AI-Powered Document Intelligence Platform
</p>
```

### Community Recognition

- **Star the repository**: Show support on GitHub
- **Share success stories**: Help others discover Swoop
- **Contribute back**: Bug fixes and features benefit everyone
- **Sponsor development**: Support ongoing development

## Legal Disclaimers

### No Warranty

Swoop is provided "as is" without warranty of any kind. While we strive for quality and reliability, we cannot guarantee:

- **Functionality**: That Swoop will meet your specific needs
- **Availability**: Uninterrupted or error-free operation
- **Security**: Freedom from vulnerabilities or attacks
- **Data safety**: Protection against data loss or corruption

### Limitation of Liability

The Swoop contributors shall not be liable for:

- **Direct damages**: Financial losses from using Swoop
- **Indirect damages**: Lost profits, business interruption
- **Data loss**: Corruption or loss of your documents or data
- **Security breaches**: Unauthorized access to your systems

### Export Compliance

Swoop may be subject to export control laws. Users are responsible for:

- **Compliance**: Following applicable export/import laws
- **Restrictions**: Not using in prohibited countries/entities
- **Encryption**: Understanding cryptographic export rules

## Getting Legal Help

### For Users

If you need legal advice about using Swoop:

1. **Consult your lawyer**: For specific legal questions
2. **Company legal team**: For enterprise use decisions
3. **Professional advice**: For complex licensing scenarios

### For Contributors

If you have questions about contributing:

1. **Employment agreement**: Check your company's IP policies
2. **Legal review**: For significant contributions
3. **Contact us**: legal@swoop.dev for specific questions

## License Changes

### Future Versions

- **Current commitment**: Swoop will remain under MIT license
- **Contributor agreement**: Existing contributions stay MIT
- **Future changes**: Would require community consensus
- **Grandfather clause**: Previous versions always MIT

### Version-Specific Licensing

- **All versions**: Currently under MIT license
- **Compatibility**: All versions compatible with each other
- **Upgrade path**: No license barriers to upgrading

---

## Summary

The MIT license makes Swoop:

- **Free to use**: No cost for any use case
- **Business friendly**: No copyleft restrictions
- **Legally clear**: Well-understood and court-tested
- **Community driven**: Encourages contribution and collaboration

For the complete legal text, see the [LICENSE](../LICENSE) file in the repository root.

**Questions?** Contact us at legal@swoop.dev or open a discussion on GitHub.