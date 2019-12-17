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

function jsPreferencesToWasmPreferences(jsPreferences: SearchPreferences): any {
  return {
    and: jsPreferences.and,
    caseSensitive: jsPreferences.caseSensitive,
    exact: jsPreferences.exact,
    includedSources: jsPreferences.toSearch.includeSource,
    includedBooks: {
      ot: Object.entries(jsPreferences.toSearch.ot).reduce((acc, [k,v]) => { if (v) {acc.push(k); return acc; }}, []),
      nt: Object.entries(jsPreferences.toSearch.nt).reduce((acc, [k,v]) => { if (v) {acc.push(k); return acc; }}, []),
      bom: Object.entries(jsPreferences.toSearch.bom).reduce((acc, [k,v]) => { if (v) {acc.push(k); return acc; }}, []),
      pogp: Object.entries(jsPreferences.toSearch.pogp).reduce((acc, [k,v]) => { if (v) {acc.push(k); return acc; }}, []),
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
