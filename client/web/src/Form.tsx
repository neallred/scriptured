import * as React from 'react';

import Preferences, { SearchPreferences } from './Preferences';
import GearSvg from './GearSvg';

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
  const [preferencesOpen, setPreferencesOpen] = React.useState(false);
  const showPreferences = React.useCallback(() => setPreferencesOpen(true), []);
  const hidePreferences = React.useCallback(() => setPreferencesOpen(false), []);
  const inputRef = React.useRef(null);
  React.useEffect(() => {
    setTimeout(() => {
      if (inputRef.current) {
        inputRef.current.focus();
      }
    });
  }, []);

  return <div>
    <div style={{
      display: 'flex',
      alignItems: 'center',
      margin: '0 auto 20px',
      width: '320px',
    }}>
      <input
        placeholder="Search scriptures"
        value={searchTerm}
        onChange={e => setSearchTerm(e.target.value)}
        ref={inputRef}
        style={{
          flex: '1 1 auto',
          maxWidth: 'calc(100% - 47px)',
        }}
      />
      <GearSvg onClick={showPreferences} style={{ marginLeft: '5px' }} />
    </div>
    {preferencesOpen &&
      <Preferences
        preferences={preferences}
        setPreferences={setPreferences}
        hidePreferences={hidePreferences}
      />
    }
  </div>
}
