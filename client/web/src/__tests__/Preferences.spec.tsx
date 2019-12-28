import { loadPreferences } from '../Preferences';

const mockGetItem = (val: any) => Object.defineProperty(window.localStorage, 'getItem', {
  value: jest.fn(() => JSON.stringify(val)),
});
describe('Preferences', () => {
  let tmpStorage = localStorage;
  beforeEach(() => {
    tmpStorage = localStorage;
    const localStorageMock = {
      getItem: jest.fn(() => null),
      setItem: jest.fn(() => null),
      length: 0,
      clear: jest.fn(() => null),
      key: jest.fn(() => null),
      removeItem: jest.fn(() => null),
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock
    });
  });

  afterEach(() => {
    Object.defineProperty(window, 'localStorage', {
      value: tmpStorage
    });
  });
  describe('loadPreferences', () => {
    it('loads default preferences when none are saved', () => {
      const preferences = loadPreferences();
      expect(preferences.caseSensitive).toBe(false);
    });

    it('uses saved preferences when their types match default types', () => {
      mockGetItem({ caseSensitive: true })
      const preferences = loadPreferences();
      expect(preferences.caseSensitive).toBe(true);
    });

    it('prefers default over saved values of the wrong type', () => {
      mockGetItem({ caseSensitive: 'true' })
      const preferences = loadPreferences();
      expect(preferences.caseSensitive).toBe(false);
    });

    it('does not add properties that are not in default preferences', () => {
      mockGetItem({ I_DONT_EXIST: true });
      const preferences = loadPreferences();
      expect((preferences as any).I_DONT_EXIST).toBe(undefined);
    });

    it('deep merges existing objects correctly', () => {
      mockGetItem({toSearch: {includeSource: { ot: false, I_DONT_EXIST: 'true' } } });
      const preferences = loadPreferences();
      expect((preferences as any).toSearch.includeSource.I_DONT_EXIST).toBe(undefined);
      expect(preferences.toSearch.includeSource.ot).toBe(false);
      expect(preferences.toSearch.includeSource.nt).toBe(true);
    });

    it('replaces arrays wholesale', () => {
      mockGetItem({toSearch: {dc: {range: [3, 44] } } });
      const preferences = loadPreferences();
      expect((preferences as any).toSearch.dc.range).toEqual([3, 44]);
    });
  });
});

