'use client';

import { useState } from 'react';

interface QuantitySelectorProps {
  value: number;
  onChange: (value: number) => void;
  min?: number;
  max?: number;
  disabled?: boolean;
}

export function QuantitySelector({
  value,
  onChange,
  min = 1,
  max = 99,
  disabled = false,
}: QuantitySelectorProps) {
  const [isUpdating, setIsUpdating] = useState(false);

  const handleDecrease = async () => {
    if (value > min && !disabled) {
      setIsUpdating(true);
      await onChange(value - 1);
      setIsUpdating(false);
    }
  };

  const handleIncrease = async () => {
    if (value < max && !disabled) {
      setIsUpdating(true);
      await onChange(value + 1);
      setIsUpdating(false);
    }
  };

  const handleInputChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = parseInt(e.target.value, 10);
    if (!isNaN(newValue) && newValue >= min && newValue <= max) {
      setIsUpdating(true);
      await onChange(newValue);
      setIsUpdating(false);
    }
  };

  return (
    <div className="inline-flex items-center border border-neutral-300 rounded-md bg-white">
      <button
        onClick={handleDecrease}
        disabled={disabled || isUpdating || value <= min}
        className="px-3 py-2 text-neutral-600 hover:bg-neutral-50 disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
        aria-label="Decrease quantity"
      >
        <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
          <path fillRule="evenodd" d="M3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clipRule="evenodd" />
        </svg>
      </button>
      <input
        type="number"
        value={value}
        onChange={handleInputChange}
        disabled={disabled || isUpdating}
        min={min}
        max={max}
        className="w-12 text-center border-x border-neutral-300 py-2 text-sm font-medium text-neutral-900 focus:outline-none disabled:opacity-40"
        aria-label="Quantity"
      />
      <button
        onClick={handleIncrease}
        disabled={disabled || isUpdating || value >= max}
        className="px-3 py-2 text-neutral-600 hover:bg-neutral-50 disabled:opacity-40 disabled:cursor-not-allowed transition-colors"
        aria-label="Increase quantity"
      >
        <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
          <path fillRule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clipRule="evenodd" />
        </svg>
      </button>
    </div>
  );
}
