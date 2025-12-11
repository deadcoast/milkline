// Farmer buddy state management store
import { writable } from "svelte/store";
import type { FarmerStateData, FarmerState, FarmerExpression } from "../types";

const initialFarmerState: FarmerStateData = {
  currentState: "idle",
  message: null,
  expression: {
    eyes: "neutral",
    mouth: "neutral",
  },
};

function createFarmerStore() {
  const { subscribe, set, update } =
    writable<FarmerStateData>(initialFarmerState);

  return {
    subscribe,
    transition: (newState: FarmerState, message: string | null = null) => {
      update((state) => {
        const expression = getExpressionForState(newState);
        return {
          currentState: newState,
          message,
          expression,
        };
      });
    },
    setExpression: (expression: Partial<FarmerExpression>) => {
      update((state) => ({
        ...state,
        expression: { ...state.expression, ...expression },
      }));
    },
    prompt: (question: string) => {
      update((state) => ({
        currentState: "prompting",
        message: question,
        expression: {
          eyes: "neutral",
          mouth: "talk-1",
        },
      }));
    },
    celebrate: (message: string = "Great!", duration: number = 2000) => {
      update((state) => ({
        currentState: "celebrating",
        message,
        expression: {
          eyes: "neutral",
          mouth: "smile",
        },
      }));

      // Auto-return to idle after duration
      setTimeout(() => {
        update((state) => ({
          ...state,
          currentState: "idle",
          message: null,
          expression: {
            eyes: "neutral",
            mouth: "neutral",
          },
        }));
      }, duration);
    },
    showError: (message: string) => {
      update((state) => ({
        currentState: "error",
        message,
        expression: {
          eyes: "neutral",
          mouth: "neutral",
        },
      }));
    },
    reset: () => set(initialFarmerState),
  };
}

function getExpressionForState(state: FarmerState): FarmerExpression {
  switch (state) {
    case "idle":
      return { eyes: "neutral", mouth: "neutral" };
    case "listening":
      return { eyes: "neutral", mouth: "smile" };
    case "prompting":
      return { eyes: "neutral", mouth: "talk-1" };
    case "celebrating":
      return { eyes: "neutral", mouth: "smile" };
    case "error":
      return { eyes: "neutral", mouth: "neutral" };
    default:
      return { eyes: "neutral", mouth: "neutral" };
  }
}

export const farmerStore = createFarmerStore();
