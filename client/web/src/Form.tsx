import * as React from 'react';

import Preferences, { SearchPreferences } from './Preferences';

interface FormProps {
  searchTerm: string;
  setSearchTerm: (searchTerm: string) => void;
  preferences: SearchPreferences;
  setPreferences: (preferences: SearchPreferences) => void;
};

export default function Form({
  searchTerm,
  setSearchTerm,
  preferences,
  setPreferences,
}: FormProps) {
  return <div>
    <input
      placeholder="Enter a search"
      value={searchTerm}
      onChange={e => setSearchTerm(e.target.value)}
    />
    <Preferences preferences={preferences} setPreferences={setPreferences}/>
  </div>
}
