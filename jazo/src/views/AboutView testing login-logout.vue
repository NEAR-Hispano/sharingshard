<template>
  <div class="about">
    <h1 class="subheading grey--text">Login</h1>
    <p>Probando login/logout</p>
    <v-container class="my-5">

      <p> ------  {{ statusT }}   ----- </p>

      <v-btn class="pa-2"
        depressed
        color="primary"
        @click=makeLogin
      >
        Login
      </v-btn>
      
      <v-spacer></v-spacer>
      
      <v-btn class="pa-2"
        depressed
        color="primary"
        @click=makeLogout
      >
        Logout
      </v-btn>

      <v-spacer></v-spacer>
      
      <v-btn class="pa-2"
        depressed
        color="primary"
        @click=statusStatus
      >
        Status
      </v-btn>


  </v-container>
</div>


</template>

<script>
  import * as nearAPI from 'near-api-js'
  const { connect, WalletConnection, keyStores } = nearAPI;

  const CONTRACT_ID = "dev-1658426475128-11579451230587";
  const config = {
    networkId: 'testnet',
    keyStore: new keyStores.BrowserLocalStorageKeyStore(),
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org',
    helperUrl: 'https://helper.testnet.near.org',
    explorerUrl: 'https://explorer.testnet.near.org'
  };

  export default {
    name: "AboutView",




    data(){
      return {
        result_num: 0,
        result_string: "",
        exp_info: "",
        statusT: "inicial"
      }
    },

    methods: {
      async makeLogout(){
        const near = await connect(config);
        const wallet = new WalletConnection(near, 'SharingShard');

/*        const contract = new Contract(wallet.account(), CONTRACT_ID, {
            viewMethods: ['get_experience'],
            changeMethods:  ['set_pov'],
            sender: wallet.account()
        });
*/
        this.statusT = "antes del Logout: " + wallet.getAccountId();

        console.log( "Antes del logout: " + wallet.getAccountId());
        wallet.signOut();
        console.log( "Despues del logout: " + wallet.getAccountId());
        console.log( "Logout " ); 
        this.statusT = this.statusT + "Despues del Logout: " + wallet.getAccountId();

      },


      async makeLogin() {
          const near = await connect(config);
          const wallet = new WalletConnection(near, 'SharingShard');

          wallet.requestSignIn( CONTRACT_ID, 'SharingShard');
          console.log( "Logout: "+ wallet.getAccountId());
          this.statusT = "Just Logged: " + wallet.getAccountId();


      },

      async statusStatus(){
          const near = await connect(config);
          const wallet = new WalletConnection(near, 'SharingShard');
          
          

          const sss = wallet.isSignedIn();
          if ( sss )
            this.statusT = "Logged: " + wallet.getAccountId();
          else
            this.statusT = "NOT Logged";
      },
    },
  }

</script>