# Playbook
There is one JSON file per locale, using names formed like
`{two lower case letters for language}-{two capitalized letters for country}`
using ISO 639-1 and ISO 3166-1 alpha 2.

JSON files contain text translations from a key to a translation value. Every
displayed text must be listed in the locales and the code must load it from
there. There should be no hard-coded strings for user-facing tests at all.

Translated texts are grouped by the page where they occur. A key should clearly
describe where the text is used. Think "title" and not "Bouncy Feet". All keys
in all locale files are in English because it is the main development language
for this project.

When the same word or phrase occurs in multiple places, it is duplicated.
Reasoning: Locales are more than simple word-to-word translation by the
dictionary. It is hard to predict which word needs a different translation in a
different context. Therefore, different occurrence in the app = different
translation entry.

# Contributions

Creating or updating a locale requires no programming knowledge.
Contributions of people who are deeply familiar with a language they want to add
or improve are welcome!
- If you have a correction, please open an issue or a pull request and describe
  briefly why the old translation is wrong.
- To add a new language, copy an existing file, rename it and then translate all
  the words. Don't translate the keys, which are always in English.