<script>
  import { locale, coachLocale } from '$lib/i18n';

  /**
   * @typedef {Object} Props
   * @prop {string} name
   */

  /** @type {Props}*/
  let { name } = $props();

  /**
   * @param {string} enStart
   * @param {string} enEnd
   * @param {string} deStart
   * @param {string} deEnd
   * @returns {{start: TranslatedText; end: TranslatedText}}
   */
  function entry(enStart, enEnd, deStart, deEnd) {
    return {
      start: {
        de: deStart,
        en: enStart,
      },
      end: {
        de: deEnd,
        en: enEnd,
      },
    };
  }

  /** @type {{start: TranslatedText; end: TranslatedText}[]} */
  export const phrases = [
    entry(
      'It looks like',
      'is enjoying themself :D',
      '',
      'sieht zufrieden aus :D'
    ),
    entry(
      '',
      'is humming and dancing on the side, with a smile on their face :)',
      '',
      'summt und tanzt vor sich hin, mit einem zufriedenen Lächeln im Gesicht :)'
    ),
  ];

  const index = Math.floor(Math.random() * phrases.length);
</script>

{phrases[index].start[coachLocale($locale)]}
{name}
{phrases[index].end[coachLocale($locale)]}
