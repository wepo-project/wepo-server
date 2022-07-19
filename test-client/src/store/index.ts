import { createStore } from "vuex";
import { UserData } from "../data";

interface MyStore {
    user: UserData | null
}

const store = createStore<MyStore>({
    state() {
        return {
            user: null, 
        }
    },
    mutations: {
        changeUser(state, payload) {
            if (payload && Object.keys(payload).length) {
                state.user = payload;
            }
        },
        logout(state, _payload) {
            state.user = null
        }
    }
})

export default store;