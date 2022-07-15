import { createStore } from "vuex";

export default createStore({
    state() {
        return {
            user: null as (UserData | null), 
        }
    },
    mutations: {
        changeUser(state, payload) {
            state.user = payload;
        }
    }
})