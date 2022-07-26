<!--

Modify to show total reward for the experience
Modify to show the assigned amount to each PoV
Validate to avoid reward more than the available records

Create a cli call to update:

  -- activate an experience and assign the amount of rewards  -- it is done in the experience page
  -- available rewards for the experience
  -- rewards assigned to each PoV
  -- pay to each user the awarded rewards 


Mode of operations:

  -- for all roles
    show the status of the experience

  -- a) as a owner of the experience
    allows to assign the reward and close the experience
      record who won the reward in the experience
      when the experience is active shows the icon to assign the reward, otherwise the icon is not shown


  -- b) as a visitor for the experience

-->


<template>
<v-main fluid>


  <v-container>
    <v-col >
      <h1>Points of View</h1>
    </v-col>
    <v-col md-12 class="text-xs-center">
      <p>Participate in SharingShard with your ideas</p>
    </v-col>
  </v-container>
    
  <v-container fluid>
      <v-row><b>Discover the value in this learning experience:</b></v-row>
      <v-row>Experience Id: {{exp_id}}</v-row>
      <v-row>Title: {{ exp_info.title }}</v-row>
      <v-row>Status: {{ exp_info.status}}</v-row>

      <v-row style="height: 500px;">
        <v-col md="6">
                   {{urlWithTime}}   
          <LazyYoutube
            ref="youtubeLazyVideo"
            max-width="720px"
            aspect-ratio="16:9"
            thumbnail-quality="standard"
            :src=urlWithTime
          />


    <div class="buttons">
      <v-btn @click="handleClick('playVideo', 'youtubeLazyVideo')">
        Play
      </v-btn>
      <v-btn @click="handleClick('pauseVideo', 'youtubeLazyVideo')">
        Pause
      </v-btn>
      <v-btn @click="handleClick('stopVideo', 'youtubeLazyVideo')">
        Stop
      </v-btn>
      <v-btn @click="handleClick('resetView', 'youtubeLazyVideo')">
        Reset
      </v-btn>
    </div>

        </v-col>


        <v-col md="6">
          <v-container id="input-usage" >
            <v-row>
              Time: 
              <v-text-field v-model="exp_info.time" readonly>
              Label="Introduce the timemark where you define the moment in the video"
              </v-text-field>
            </v-row>
          
            <v-row>
              <v-col
                >
                <v-textarea
                  filled
                  v-model="exp_info.moment"                         
                  label="Display the Moment for the Experience"
                  value=ss_moment
                  rows="5"
                  readonly
                ></v-textarea>

              </v-col>
            </v-row>

            <v-row v-if=!this.iAmOwner>
              <v-col
                >
                <v-textarea
                  filled
                  v-model="ss_pov"                         
                  name="input-7-4"
                  label="Create a Point of View about this challenge"
                  value=ss_pov
                  rows="5"
                ></v-textarea>

              </v-col>
            </v-row>
          </v-container>

        </v-col>
      </v-row>
      
  </v-container>

  <v-divider class="my-8"></v-divider>

  <v-spacer class="my-8" />
  <v-container>
    
      <v-row>
        <v-col md="6">
          <p>URL del video:</p><span>{{urlWithTime}}</span>

        </v-col>

        <v-col md="6">  
          <v-row justify="space-around">
            <v-col md="2"> 
              <v-btn
                depressed
                color="primary"
                @click=clickea_edit_pov()
                v-show=future_feature
              >
                Edit
              </v-btn>
            </v-col>

            <v-col md="2">
              <v-btn
                depressed
                color="primary"
                @click=clickea_insert_pov()
                v-show=!iAmOwner

              >
                Insert
              </v-btn>
            </v-col> 



            <v-col md="2"> 
              <v-btn
                depressed
                color="primary"
                @click=clickea_delete_pov()
                v-show=future_feature

              >
                Delete
              </v-btn>
            </v-col> 
          </v-row>
        </v-col>
      </v-row>

  </v-container>
<!----------------------------------------------->
  <v-container fluid :key="componentKey">
    <div>
      <h2>List Points of View</h2>
      <div>
  <v-data-table
    :headers="headers"
    :items="PoV"
    sort-by="name"
    class="elevation-1"
  >
    <template v-slot:top>
      <v-toolbar
        flat
      >
        <v-toolbar-title>Rewards available for the best Point of View: {{exp_info.reward}} </v-toolbar-title>
        <v-divider
          class="mx-4"
          inset
          vertical
        ></v-divider>
        <v-spacer></v-spacer>
        <v-dialog
          v-model="dialog"
          max-width="500px"
        >
          <template v-slot:activator="{ on, attrs }">
            <v-btn
              color="primary"
              dark
              class="mb-2"
              v-bind="attrs"
              v-on="on"
              v-show=future_feature
            >
              New Point of View
            </v-btn>
          </template>
          <v-card>
            <v-card-title>
              <span class="text-h5">{{ formTitle }}</span>
            </v-card-title>

            <v-card-text>
              <v-container>
                <v-row>
                  <v-col
                    cols="12"
                    sm="6"
                    md="4"
                  >
                    <v-text-field
                      v-model="editedItem.name"
                      label="PoV Number"
                    ></v-text-field>
                  </v-col>
                  <v-col
                    cols="12"
                    sm="6"
                    md="4"
                  >
                    <v-text-field
                      v-model="editedItem.owner"
                      label="Owner"
                    ></v-text-field>
                  </v-col>
                  <v-col
                    cols="12"
                    sm="6"
                    md="4"
                  >
                    <v-text-field
                      v-model="editedItem.video"
                      label="Video"
                    ></v-text-field>
                  </v-col>
                </v-row>
              </v-container>
            </v-card-text>

            <v-card-actions>
              <v-spacer></v-spacer>
              <v-btn
                color="blue darken-1"
                text
                @click="close"
              >
                Cancel
              </v-btn>
              <v-btn
                color="blue darken-1"
                text
                @click="save"
              >
                Save
              </v-btn>
            </v-card-actions>
          </v-card>
        </v-dialog>
      </v-toolbar>
    </template>
    <template v-slot:[`item.actions`]="{ item }">
      <v-icon
        small
        class="mr-2"
        @click="editItem(item)"
        v-show=iAmOwner
      >
        mdi-account-cash
      </v-icon>
    </template>

<!--
    <template v-slot:no-data>
      <v-btn
        color="primary"
        @click="disp_experiences_for_pov"
      >
        Reset
      </v-btn>
    </template>
-->

  </v-data-table>
      </div>
    </div>
  </v-container>
<!----------------------------------------------->

  <div class="text-center">
    <v-dialog
      v-model="dialog_reward"
      width="500"
    >
      <v-card>
        <v-card-title class="text-h5 grey lighten-2">
          Rewards Assigment
        </v-card-title>

        <v-card-text>
          <p>PoV: {{ editedIndex }} </p>
          <p>Reward: {{ editedItem.reward }}</p>
          <p>Assigned to: {{ editedItem.owner }} </p>
          <!--<p>The award for this participation is: <v-text-field v-model="ss_pov_moment_award" reverse></v-text-field></p>-->
          <!--<p>Rewards available for assignment: {{ editedItem.reward  - ss_pov_moment_award }}</p>-->
        </v-card-text>

        <v-divider></v-divider>

        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn
            color="primary"
            dark
            @click=payRewardAndClose()
          >
            Assign
          </v-btn>
          <v-btn
            color="primary"
            dark
            @click="dialog_reward = false"
          >
            Cancel
          </v-btn>

        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</v-main>
</template>
    
<script>

import * as nearAPI from 'near-api-js'
const { connect, WalletConnection, keyStores, Contract } = nearAPI;



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
    name: "PoView",

    created(){

//    updated(){
      // this.disp_pov();
      /*

      start and stop the video
      https://support.gospacecraft.com/hc/en-us/articles/204744184-Set-Specific-Start-and-End-Times-for-Your-Video

      console.log("POV created: " + window.location.href);
      console.log(this.$route.params);
      const cual = this.$route.params.video_id;
      console.log( cual );
      console.log(this.$route.params.video_id);
      */
      this.exp_id = this.$route.params.video_id
      this.disp_experiences_for_pov( this.exp_id );
      // this.display_PV = true;
    },

    computed: {
      urlWithTime: function(){
        // return this.exp_info.url + "?start=" + parseInt(this.exp_info.time) + "&end=" + parseInt(this.exp_info.time)+60;
        return this.exp_info.url + "?t=" + this.exp_info.time;

    },

      formTitle () {
        return this.editedIndex === -1 ? 'New Item' : 'Edit Item'
      },
    },

    watch: {
      dialog (val) {
        val || this.close()
      },
      dialog_reward (val) {
        val || this.close()
      },
    },


    data:() => ({

      //return {
        exp_id: 0,  // is the video_no of the experience
        exp_info: "",

        dialog: false,
        dialog_reward: false,

        iAmOwner: false,
        future_feature: false,

        poview: "",
        title: "",
        time: 10,
        moment: "cadena de caracteres",

        btn_insert: "Insert",
        btn_edit: "Edit",
        btn_delete: "Delete",
        textarea: null,

        ss_moment: "",

        ss_pov: "",
        ss_pov_moment_time: 33,
        ss_pov_moment_award: 0,

        headers: [
          {
            text: 'Points of View Number',
            align: 'start',
            sortable: false,
            value: 'name',
          },
          { text: 'Participant', value: 'owner' },
          { text: 'Point of View', value: 'PoV' },
          { text: 'Reward', value: 'reward'},
          { text: 'Actions', value: 'actions', sortable: false },
        ],

        /*
          {
            name: 'Video 1',
            owner: 'ss2022_jc.testnet',  //wallet .
            video_id: 2,
            PoV: "PoV 1 en el video 2 de Jorge",
          }
          */
        PoV:[ ],

        editedIndex: -1,
        editedItem: {
          name: '',
          owner: "",
          PoV: "",
          reward: 0,
        },
        defaultItem: {
          name: '',
          owner: "",
          PoV: "",
          reward: 0,
        },

        componentKey: 0,
      }),

    methods: {

      handleClick(event, ref) {
          this.$refs[ref][event]();
      },

      forceRerender() {
        this.componentKey += 1;
      },

      clickea_insert_pov(){
        console.log( "clickea_insert_pov -- Captura: " + this.ss_pov);
        this.insert_pov( this.exp_id );
      },
      clickea_edit_pov(){
        console.log( "clickea_edit_pov -- Captura: " + this.ss_pov);
      },
      clickea_delete_pov(){
        console.log( "clickea_delete_pov -- Captura: " + this.ss_pov );
      },

      async insert_pov( video_id ){

          const near = await connect(config);
          const wallet = new WalletConnection(near, 'SharingShard');

          const contract = new Contract(wallet.account(), CONTRACT_ID, {
              viewMethods: ['get_experience',
              'get_number_of_experiences', 
              'get_user_exp'],
              changeMethods:  ['set_pov'],
              sender: wallet.account()
          });


        console.log( "video id: " + video_id);
        this.exp_info = await contract.get_experience({
          video_n: video_id
        });

        this.inserting_pov = this.exp_info;

        // alert ("hola: " + wallet.getAccountId() + this.exp_info.owner );
        if ( this.iAmOwner ){
          alert( "you, as an owner, are allowed to provide a Point of View");
        }

          console.log( "Experience: " + this.exp_info);
          console.log( this.exp_info.pov.length );
          console.log( wallet.getAccountId());

          if ( Object.entries(this.exp_info.pov ).length == 0 )  // No hay ningun comentario
            console.log( "insertando first PoV...... ")
          else
            console.log( "insertando PoV...... ")

          this.ss_pov_moment_award = this.exp_info.reward;
          this.exp_info = await contract.set_pov({
              video_n: video_id,
              pov: this.ss_pov,
              date: 10,
          });
          console.log("Comentario insertado" + this.exp_info);
      },

      async disp_experiences_for_pov( video_id ){
        const near = await connect(config);
        const wallet = new WalletConnection(near, 'SharingShard');

        const contract = new Contract(wallet.account(), CONTRACT_ID, {
          viewMethods:  ['get_number_of_experiences', 'get_user_exp','get_experience', 'set_pov'],
          sender: wallet.account()
        });

        this.exp_info = await contract.get_experience({
          video_n: video_id
        });

          // alert( "this.exp_info.owner: " + this.exp_info.owner);
          // alert( "accountId: " + wallet.getAccountId());

        if ( wallet.getAccountId() == this.exp_info.owner )
          this.iAmOwner = true;


  /*for ( const [key, value] of Object.entries(this.exp_info.pov )) {
    console.log (key, value);
  }*/

          let PoVitem = {};
          const PoV_listing = [];
         //this.PoV.pop();
          for ( let i = 0 ; i < Object.entries(this.exp_info.pov ).length ; i++  ){
            PoVitem = {};
            PoVitem.name = "Point of View: " + i;
            PoVitem.owner = Object.entries(this.exp_info.pov)[i][0];
            PoVitem.video_id = i;
            PoVitem.PoV = Object.entries(this.exp_info.pov)[i][1];
            PoVitem.reward = this.exp_info.reward;

            PoV_listing.push( PoVitem);
          }

          this.PoV = PoV_listing;

          this.forceRerender();

        },

        close_reward () {
          this.dialog_reward = false
        },

        editItem (item) {
          this.editedIndex = this.PoV.indexOf(item)
          this.editedItem = Object.assign({}, item)
          this.dialog_reward = true
        },

        close () {
          this.dialog = false
          this.$nextTick(() => {
            this.editedItem = Object.assign({}, this.defaultItem)
            this.editedIndex = -1
          })
        },

        save () {
          if (this.editedIndex > -1) {
            Object.assign(this.PoV[this.editedIndex], this.editedItem)
          } else {
            this.PoV.push(this.editedItem)
          }
          this.close()
        },

      async payRewardAndClose(){

        const video_n = this.exp_id;
        const participant = this.editedItem.owner;

        //alert( "Video: " + video_n + " Participant: " + participant);
        const near = await connect(config);
        const wallet = new WalletConnection(near, 'SharingShard');

        const contract = new Contract( wallet.account(), CONTRACT_ID, 
        { 
          changeMethods:  ['pay_reward'],
          viewMethods: ['get_experience'],
          sender: wallet.account(),
        });


        this.exp_info = await contract.get_experience({
          video_n: video_n // video_id
        });

        this.expInfo = await contract.pay_reward(
          { 
            experience_number: video_n,
            wallet: participant,
          }, 
          300000000000000,
          0
        )
        .catch((err) => {
          alert ( err );
          return err;
        }
        );

        this.statusT = "pay_reward result: " + this.expInfo;
        console.log( this.expInfo );

        this.statusT = await contract.get_experience({
          video_n: video_n // video_id
        });
        this.dialog_reward = false;
      },
    },    // methods:
}   // export
</script>
