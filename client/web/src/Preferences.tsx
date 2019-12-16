import * as React from 'react';
import { debounce } from './utils';

type SectionNumbers = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 123 | 124 | 125 | 126 | 127 | 128 | 129 | 130 | 131 | 132 | 133 | 134 | 135 | 136 | 137 | 138; 

const bookOrders = {
  ot: [
    "Genesis",
    "Exodus",
    "Leviticus",
    "Numbers",
    "Deuteronomy",
    "Joshua",
    "Judges",
    "Ruth",
    "1 Samuel",
    "2 Samuel",
    "1 Kings",
    "2 Kings",
    "1 Chronicles",
    "2 Chronicles",
    "Ezra",
    "Nehemiah",
    "Esther",
    "Job",
    "Psalms",
    "Proverbs",
    "Ecclesiastes",
    "Solomon's Song",
    "Isaiah",
    "Jeremiah",
    "Lamentations",
    "Ezekiel",
    "Daniel",
    "Hosea",
    "Joel",
    "Amos",
    "Obadiah",
    "Jonah",
    "Micah",
    "Nahum",
    "Habakkuk",
    "Zephaniah",
    "Haggai",
    "Zechariah",
    "Malachi",
  ],
  nt: [
    "Matthew",
    "Mark",
    "Luke",
    "John",
    "Acts",
    "Romans",
    "1 Corinthians",
    "2 Corinthians",
    "Galatians",
    "Ephesians",
    "Philippians",
    "Colossians",
    "1 Thessalonians",
    "2 Thessalonians",
    "1 Timothy",
    "2 Timothy",
    "Titus",
    "Philemon",
    "Hebrews",
    "James",
    "1 Peter",
    "2 Peter",
    "1 John",
    "2 John",
    "3 John",
    "Jude",
    "Revelation",
  ],
  bom: [
    "1 Nephi",
    "2 Nephi",
    "Jacob",
    "Enos",
    "Jarom",
    "Omni",
    "Words of Mormon",
    "Mosiah",
    "Alma",
    "Helaman",
    "3 Nephi",
    "4 Nephi",
    "Mormon",
    "Ether",
    "Moroni",
  ],
  pogp: [
    "Moses",
    "Abraham",
    "Joseph Smith—Matthew",
    "Joseph Smith—History",
    "Articles of Faith",
  ],
}

interface Preferences {
  and: boolean;
  or: boolean;
  caseSensitive: boolean;
  exact: boolean
  toSearch: {
    includeSource: {
      ot: boolean;
      nt: boolean;
      bom: boolean;
      dc: boolean;
      pogp: boolean;
    }
    ot: {
      "Genesis": boolean;
      "Exodus": boolean;
      "Leviticus": boolean;
      "Numbers": boolean;
      "Deuteronomy": boolean;
      "Joshua": boolean;
      "Judges": boolean;
      "Ruth": boolean;
      "1 Samuel": boolean;
      "2 Samuel": boolean;
      "1 Kings": boolean;
      "2 Kings": boolean;
      "1 Chronicles": boolean;
      "2 Chronicles": boolean;
      "Ezra": boolean;
      "Nehemiah": boolean;
      "Esther": boolean;
      "Job": boolean;
      "Psalms": boolean;
      "Proverbs": boolean;
      "Ecclesiastes": boolean;
      "Solomon's Song": boolean;
      "Isaiah": boolean;
      "Jeremiah": boolean;
      "Lamentations": boolean;
      "Ezekiel": boolean;
      "Daniel": boolean;
      "Hosea": boolean;
      "Joel": boolean;
      "Amos": boolean;
      "Obadiah": boolean;
      "Jonah": boolean;
      "Micah": boolean;
      "Nahum": boolean;
      "Habakkuk": boolean;
      "Zephaniah": boolean;
      "Haggai": boolean;
      "Zechariah": boolean;
      "Malachi": boolean;
    }
    nt: {
      "Matthew": boolean;
      "Mark": boolean;
      "Luke": boolean;
      "John": boolean;
      "Acts": boolean;
      "Romans": boolean;
      "1 Corinthians": boolean;
      "2 Corinthians": boolean;
      "Galatians": boolean;
      "Ephesians": boolean;
      "Philippians": boolean;
      "Colossians": boolean;
      "1 Thessalonians": boolean;
      "2 Thessalonians": boolean;
      "1 Timothy": boolean;
      "2 Timothy": boolean;
      "Titus": boolean;
      "Philemon": boolean;
      "Hebrews": boolean;
      "James": boolean;
      "1 Peter": boolean;
      "2 Peter": boolean;
      "1 John": boolean;
      "2 John": boolean;
      "3 John": boolean;
      "Jude": boolean;
      "Revelation": boolean;
    }
    bom: {
      "1 Nephi": boolean;
      "2 Nephi": boolean;
      "Jacob": boolean;
      "Enos": boolean;
      "Jarom": boolean;
      "Omni": boolean;
      "Words of Mormon": boolean;
      "Mosiah": boolean;
      "Alma": boolean;
      "Helaman": boolean;
      "3 Nephi": boolean;
      "4 Nephi": boolean;
      "Mormon": boolean;
      "Ether": boolean;
      "Moroni": boolean;
    }
    dc: {
      range: [SectionNumbers, SectionNumbers];
    }
    pogp: {
      "Moses": boolean;
      "Abraham": boolean;
      "Joseph Smith—Matthew": boolean;
      "Joseph Smith—History": boolean;
      "Articles of Faith": boolean;
    }
  }
}

const defaultPreferences: Preferences = {
  and: true,
  or: false,
  caseSensitive: true,
  exact: true,
  toSearch: {
    includeSource: {
      ot: true,
      nt: true,
      bom: true,
      dc: true,
      pogp: true,
    },
    ot: {
      "Genesis": true,
      "Exodus": true,
      "Leviticus": true,
      "Numbers": true,
      "Deuteronomy": true,
      "Joshua": true,
      "Judges": true,
      "Ruth": true,
      "1 Samuel": true,
      "2 Samuel": true,
      "1 Kings": true,
      "2 Kings": true,
      "1 Chronicles": true,
      "2 Chronicles": true,
      "Ezra": true,
      "Nehemiah": true,
      "Esther": true,
      "Job": true,
      "Psalms": true,
      "Proverbs": true,
      "Ecclesiastes": true,
      "Solomon's Song": true,
      "Isaiah": true,
      "Jeremiah": true,
      "Lamentations": true,
      "Ezekiel": true,
      "Daniel": true,
      "Hosea": true,
      "Joel": true,
      "Amos": true,
      "Obadiah": true,
      "Jonah": true,
      "Micah": true,
      "Nahum": true,
      "Habakkuk": true,
      "Zephaniah": true,
      "Haggai": true,
      "Zechariah": true,
      "Malachi": true,
    },
    nt: {
      "Matthew": true,
      "Mark": true,
      "Luke": true,
      "John": true,
      "Acts": true,
      "Romans": true,
      "1 Corinthians": true,
      "2 Corinthians": true,
      "Galatians": true,
      "Ephesians": true,
      "Philippians": true,
      "Colossians": true,
      "1 Thessalonians": true,
      "2 Thessalonians": true,
      "1 Timothy": true,
      "2 Timothy": true,
      "Titus": true,
      "Philemon": true,
      "Hebrews": true,
      "James": true,
      "1 Peter": true,
      "2 Peter": true,
      "1 John": true,
      "2 John": true,
      "3 John": true,
      "Jude": true,
      "Revelation": true,
    },
    bom: {
      "1 Nephi": true,
      "2 Nephi": true,
      "Jacob": true,
      "Enos": true,
      "Jarom": true,
      "Omni": true,
      "Words of Mormon": true,
      "Mosiah": true,
      "Alma": true,
      "Helaman": true,
      "3 Nephi": true,
      "4 Nephi": true,
      "Mormon": true,
      "Ether": true,
      "Moroni": true,
    },
    dc: {
      range: [1, 138],
    },
    pogp: {
      "Moses": true,
      "Abraham": true,
      "Joseph Smith—Matthew": true,
      "Joseph Smith—History": true,
      "Articles of Faith": true,
    },
  },
}

interface PreferencesProps {
  preferences: Preferences
}

function loadPreferences(): Preferences {
  const savedPreferences = JSON.parse(localStorage.getItem('verilyPreferences')) as Preferences;

  if (savedPreferences) {
    return savedPreferences
  } else {
    return defaultPreferences
  }
}

function savePreferences(preferences: Preferences) {
  localStorage.setItem('verilyPreferences', JSON.stringify(preferences));
}

const debouncedSavePreferences = debounce(savePreferences);

type Bookronym = "ot" | "nt" | "bom" | "dc" | "pogp";

interface BookSourceProps {
  bookronym: Bookronym;
  title: string;
  includeSource: boolean;
  bookOrder?: string[];
  booksIncluded?: {[key: string]: boolean};
  numberRange?: number[];
  setPathValue: Function;
  setAll: (bookronym: Bookronym, includeAll: boolean) => void,
}

function BookSource({
  bookronym,
  title,
  includeSource,
  bookOrder,
  booksIncluded,
  numberRange,
  setPathValue,
  setAll,
}: BookSourceProps) {
  const [open, setOpen] = React.useState(false);
  const allIncluded = includeSource && bookOrder.every(x => booksIncluded[x]);
  return <div>
    <div>{title}</div>
    <button onClick={() => setOpen(!open)}>{open ? "See less" : "See More"}</button>
    <button onClick={() => setPathValue(['includeSource', bookronym], !includeSource)}>{includeSource ? `Exclude ${title}` : `Include ${title}`}</button>
    <button onClick={() => setAll(bookronym, !allIncluded)}>{allIncluded ? `Exclude all ${title} books` : `Include all ${title} books`}</button>
    {open && bookOrder && booksIncluded
        ?  bookOrder.map(x => {
          return <div>
            <label>
            <input
              type="checkbox"
              checked={booksIncluded[x]}
              onChange={e => setPathValue([bookronym, x], e.target.checked)}
              key={x}
            />
            {x}
          </label>


          </div>
        })
        : null
    }
    { numberRange
        ? `${numberRange[0]} ${numberRange[1]}`
        : null
    }
  </div>
}

function access(path: string[], value: any) {
  return path.reduce((acc, curr) => acc[curr] , value);
}

function deepSet(path: string[], value: any, obj: any, merge: boolean = false): any {
  const k = path[0];
  if (!k) {
    return obj;
  }
  return { 
    ...obj,
    [k]: (path.length === 1)
        ? (merge ? ({ ...obj[k], ...value }): value)
        : deepSet(path.slice(1), value, obj[k], merge)
  };
}

export default function Preferences() {
  const [preferences, setPreferences] = React.useState(loadPreferences());
  React.useEffect(() => {
    debouncedSavePreferences(preferences);
  }, [preferences]);

  const setPathValue = React.useCallback((path: string[], value: boolean | string | number) => {
    setPreferences(deepSet(['toSearch'].concat(path), value, preferences));
  }, [preferences]);

  const setAll = React.useCallback((bookronym: Bookronym, allValue: boolean) => {
    const retVal: any = {};
    setPreferences({
      ...preferences,
      toSearch: {
        ...preferences.toSearch,
        [bookronym]: Object.keys(preferences.toSearch[bookronym]).reduce((acc, curr) => { acc[curr] = allValue; return acc; }, retVal),
        includeSource: {
          ...preferences.toSearch.includeSource,
          [bookronym]: allValue,
        },
      },
    });
  }, [preferences]);
  return <div>
    Preferences
    <div>
      And search: <input type="checkbox" checked={preferences.and} onChange={e => setPreferences({...preferences, and: e.target.checked})} />
    </div>
    <div>
      Case sensitive search: <input type="checkbox" checked={preferences.caseSensitive} onChange={e => setPreferences({...preferences, caseSensitive: e.target.checked})} />
    </div>

    Books to include
    <BookSource
      bookronym="ot"
      title="Old Testament"
      includeSource={preferences.toSearch.includeSource.ot}
      bookOrder={bookOrders.ot}
      booksIncluded={preferences.toSearch.ot}
      // numberRange: number[];
      setPathValue={setPathValue}
      setAll={setAll}
    />
    Old Testament
    New Testament
    Book of Mormon
    Doctrine And Covenants
    Pearl of Great Price

    Basic
    By Book
    preferences.
    By Sub Books

  </div>
}
