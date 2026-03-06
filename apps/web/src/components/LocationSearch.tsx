import { useState, useEffect, useRef, useMemo } from "react";
import { retrieveLocations, type Location } from "@/lib/api";
import { Input } from "@/components/ui/input";

const STORAGE_KEY = "locations_cache";
const MAX_RESULTS = 50;

interface LocationSearchProps {
  onSelect?: (location: Location) => void;
  placeholder?: string;
  name?: string;
  defaultValue?: string;
}

export default function LocationSearch({ onSelect, placeholder = "Search locations...", name, defaultValue = "" }: LocationSearchProps) {
  const [locations, setLocations] = useState<Location[]>([]);
  const [searchTerm, setSearchTerm] = useState(defaultValue);
  const [selecteduic, setSelecteduic] = useState(defaultValue);
  const [isOpen, setIsOpen] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [selectedIndex, setSelectedIndex] = useState(-1);
  const dropdownRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const loadLocations = async () => {
      // Try to get from localStorage first
      const cached = localStorage.getItem(STORAGE_KEY);
      if (cached) {
        try {
          const parsedLocations = JSON.parse(cached) as Location[];
          setLocations(parsedLocations);
          setIsLoading(false);
          return;
        } catch (e) {
          console.error("Failed to parse cached locations:", e);
        }
      }

      // Fetch from API if not in localStorage
      try {
        const fetchedLocations = await retrieveLocations();
        
        setLocations(fetchedLocations);
        localStorage.setItem(STORAGE_KEY, JSON.stringify(fetchedLocations));
      } catch (e) {
        console.error("Failed to fetch locations:", e);
      } finally {
        setIsLoading(false);
      }
    };

    loadLocations();
  }, []);

  // Handle click outside to close dropdown
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const filteredLocations = useMemo(() => {
    const term = searchTerm.toLowerCase().trim();
    
    // Don't filter if search term is too short
    if (term.length < 2) {
      return [];
    }
    
    const results: Location[] = [];
    for (const location of locations) {
      if (results.length >= MAX_RESULTS) break;
      
      const display = location.display?.toLowerCase();
      if (display?.includes(term)) {
        results.push(location);
      }
    }
    
    return results;
  }, [locations, searchTerm]);

  const handleSelect = (location: Location) => {
    setSearchTerm(location.display || location.uic || "");
    setSelecteduic(location.uic || "");
    setIsOpen(false);
    setSelectedIndex(-1);
    onSelect?.(location);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (!isOpen) {
      if (e.key === "ArrowDown" || e.key === "Enter") {
        setIsOpen(true);
      }
      return;
    }

    switch (e.key) {
      case "ArrowDown":
        e.preventDefault();
        setSelectedIndex((prev) => 
          prev < filteredLocations.length - 1 ? prev + 1 : prev
        );
        break;
      case "ArrowUp":
        e.preventDefault();
        setSelectedIndex((prev) => (prev > 0 ? prev - 1 : -1));
        break;
      case "Enter":
        e.preventDefault();
        if (selectedIndex >= 0 && selectedIndex < filteredLocations.length) {
          handleSelect(filteredLocations[selectedIndex]);
        }
        break;
      case "Escape":
        setIsOpen(false);
        setSelectedIndex(-1);
        break;
    }
  };

  return (
    <div ref={dropdownRef} className="relative w-full max-w-sm">
      {name && <input type="hidden" name={name} value={selecteduic} />}
      <Input
        ref={inputRef}
        type="text"
        value={searchTerm}
        onChange={(e) => {
          setSearchTerm(e.target.value);
          setSelecteduic(""); // Clear the hidden value when typing
          setIsOpen(true);
          setSelectedIndex(-1);
        }}
        onFocus={() => setIsOpen(true)}
        onKeyDown={handleKeyDown}
        placeholder={isLoading ? "Loading locations..." : placeholder}
        disabled={isLoading}
      />
      
      {isOpen && !isLoading && (
        <div className="absolute z-50 mt-1 w-full max-h-60 overflow-auto rounded-md border border-input bg-background shadow-lg">
          {searchTerm.trim().length < 2 ? (
            <div className="px-3 py-2 text-sm text-muted-foreground">
              Type at least 2 characters to search
            </div>
          ) : filteredLocations.length === 0 ? (
            <div className="px-3 py-2 text-sm text-muted-foreground">
              No locations found
            </div>
          ) : (
            filteredLocations.map((location, index) => (
              <div
                key={location.uic || index}
                className={`cursor-pointer px-3 py-2 text-sm transition-colors ${
                  index === selectedIndex
                    ? "bg-accent text-accent-foreground"
                    : "hover:bg-accent hover:text-accent-foreground"
                }`}
                onClick={() => handleSelect(location)}
                onMouseEnter={() => setSelectedIndex(index)}
              >
                <span className="font-medium">{location.display || location.uic}</span>
                {location.display && location.uic && (
                  <span className="ml-2 text-muted-foreground">({location.uic})</span>
                )}
              </div>
            ))
          )}
        </div>
      )}
    </div>
  );
}