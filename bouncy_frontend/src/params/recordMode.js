/**
 * @param {string} param
 * @return {param is ('learn' | 'train')}
 * @satisfies {import('@sveltejs/kit').ParamMatcher}
 */
export function match(param) {
    return param === 'learn' || param === 'train';
}