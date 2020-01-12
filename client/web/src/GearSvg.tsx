import * as React from 'react';

const ACTION_KEYS: {[key: string]: true} = {
  'Enter': true,
  'Space': true,
  ' ': true,
  '': true,
}

interface GearSvgProps {
  size?: number,
  onClick: () => void,
  style?: {[key: string]: number | string},
  tabIndex?: number,
}
export default function GearSvg({
  size = 25,
  onClick,
  style,
  tabIndex = 0
}: GearSvgProps) {
  const sizePx = `${size}px`;
  const sizeShadowPx = `${size / 12}px`;
  return (<svg
    onClick={onClick}
    onKeyDown={e => {
        if (ACTION_KEYS[e.key]) {
          onClick();
        }
      }
    }
    style={{
      boxShadow: `${sizeShadowPx} ${sizeShadowPx} ${sizeShadowPx} rgba(0,0,0,0.7)`,
      borderRadius: '100%',
      ...style
    }}
    tabIndex={tabIndex}
    xmlns="http://www.w3.org/2000/svg"
    version="1.1"
    x="0px"
    y="0px"
    width={sizePx}
    height={sizePx}
    viewBox="0 0 512 512"
  >
    <path d="M468.531,310.844H512V201.125h-43.469c-44.094,0-54.594-25.469-23.5-56.594l30.781-30.781l-77.594-77.563l-30.75,30.75  c-31.156,31.156-56.656,20.594-56.563-23.438c0-0.125-0.063-0.188-0.063-0.281V0H201.172v43.625  c-0.125,43.938-25.531,54.438-56.641,23.313l-30.766-30.75L36.203,113.75l30.75,30.781c31.141,31.125,20.578,56.594-23.438,56.594H0  v109.719h43.516c44.016,0,54.578,25.469,23.438,56.625l-30.75,30.75l77.563,77.594l30.766-30.781  c31.109-31.094,56.516-20.594,56.641,23.313V512h109.672v-43.219c0-0.094,0.063-0.188,0.063-0.281  c-0.094-44.031,25.406-54.563,56.563-23.469l30.75,30.781l77.594-77.594l-30.781-30.75  C413.938,336.313,424.438,310.844,468.531,310.844z M256,352c-53.016,0-96-43-96-96s42.984-96,96-96c53,0,96,43,96,96  S309,352,256,352z"/>
  </svg>);
}
