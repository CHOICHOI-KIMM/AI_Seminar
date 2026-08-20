import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { BearingInput, SliceGeometry, SliceContactResult } from "../types/bearing";

export function useSolver() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const computeSliceGeometry = useCallback(async (input: BearingInput): Promise<SliceGeometry[] | null> => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<SliceGeometry[]>("compute_slice_geometry", { input });
      return result;
    } catch (e) {
      setError(String(e));
      return null;
    } finally {
      setLoading(false);
    }
  }, []);

  const computeHertzSingleSlice = useCallback(async (
    deltaK: number,
    rEq: number,
    eRoller: number,
    eRing: number,
    nu: number,
    sliceWidth: number,
    h1: number,
    h2: number,
  ): Promise<SliceContactResult | null> => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<SliceContactResult>("compute_hertz_single_slice", {
        deltaK,
        rEq,
        eRoller,
        eRing,
        nu,
        sliceWidth,
        h1,
        h2,
      });
      return result;
    } catch (e) {
      setError(String(e));
      return null;
    } finally {
      setLoading(false);
    }
  }, []);

  return { computeSliceGeometry, computeHertzSingleSlice, loading, error };
}
