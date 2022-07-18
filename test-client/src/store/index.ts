import { createStore } from "vuex";
import { UserData } from "../data";

export default createStore({
    state() {
        return {
            user: null as (UserData | null), 
        }
    },
    mutations: {
        changeUser(state, payload) {
            if (payload && Object.keys(payload).length) {
                state.user = payload;
            }
        }
    }
})