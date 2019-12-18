import * as ReactDOM from 'react-dom';
import * as React from 'react';

import * as wasm from "wasm-verily-client";
wasm.set_panic_hook();

import NoResult from './NoResult';
import Result from './Result';
import Form from './Form';
import { loadPreferences, SearchPreferences } from './Preferences';
import { debounce } from './utils';

interface AppProps {
}

function reduceString(acc: string[], [k, v]: [string, boolean]): string[] {
  if (v) {
    acc.push(k);
  }
  return acc;
}

function jsPreferencesToWasmPreferences(jsPreferences: SearchPreferences): any {
  const {
    caseSensitive,
    exact,
    toSearch,
  } = jsPreferences;
  return {
    and: jsPreferences.and,
    caseSensitive,
    exact,
    includedSources: toSearch.includeSource,
    includedBooks: {
      ot: Object.entries(toSearch.ot).reduce(reduceString, []),
      nt: Object.entries(toSearch.nt).reduce(reduceString, []),
      bom: Object.entries(toSearch.bom).reduce(reduceString, []),
      pogp: Object.entries(toSearch.pogp).reduce(reduceString, []),
      dc: toSearch.dc.range
    },
  };
}

const noResults: string[] = [];
const SHORTEST_SEARCH_LENGTH = 2;
function App({}: AppProps) {
  const [searchTerm, setSearchTerm] = React.useState("");
  const [preferences, setPreferences] = React.useState(loadPreferences());
  const [results, setResults] = React.useState<string[]>(noResults);

  const debouncedFullTextSearch = React.useCallback(debounce((currentSearchTerm: string, preferences: SearchPreferences) => {
    const newResults = currentSearchTerm.length >= SHORTEST_SEARCH_LENGTH
      ? wasm.full_match_search(currentSearchTerm, jsPreferencesToWasmPreferences(preferences as any))
      : [];
    setResults(newResults);
  }), []);

  React.useEffect(() => { debouncedFullTextSearch(
    searchTerm,
    preferences,
  ) }, [searchTerm, preferences]);
  const boundSetSearchTerm = React.useCallback(newTerm => setSearchTerm(newTerm), []);

  return <div>
    <Form
      searchTerm={searchTerm}
      setSearchTerm={boundSetSearchTerm}
      preferences={preferences}
      setPreferences={setPreferences}
    />
    {results.length
      ? results.map(x => <Result key={x} displayString={x} />)
      : <NoResult searchTerm={searchTerm} />
    }
  </div>

}

ReactDOM.render(<App />, document.getElementById('verily-root'));
