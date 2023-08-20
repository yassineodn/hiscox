<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_iframe srchttpswww.googletagmanager.co_633b40</name>
   <tag></tag>
   <elementGuidId>820c4dbc-a874-4103-9cea-258c41e21004</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body.page-prequotequestions.created-from-newbusiness.page-number-1.package-12462.sale-stage-quickquote.site-public.theme-responsive.safari</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>9e3972e1-3b61-42ef-8a82-bcef9189914a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>page-prequotequestions created-from-newbusiness page-number-1 package-12462 sale-stage-quickquote site-public theme-responsive safari</value>
      <webElementGuid>c30e18bd-671c-4528-a7ef-f1ea5ece2733</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>


&lt;iframe src=&quot;https://www.googletagmanager.com/ns.html?id=GTM-NZRGST&quot;
height=&quot;0&quot; width=&quot;0&quot; style=&quot;display:none;visibility:hidden&quot;>&lt;/iframe>

        
        
        
Vous avez une question ? 01 89 07 75 97Appel gratuit | Du lundi au vendredi 08h30 - 20h00 hors jours fériés

    
        
            
                
                    Toggle navigation
                    
                    
                    
                

                    
                        
                        
                    
            
Votre profilVotre besoin Votre devis Vous êtes assuré


            

Vous avez une question ? 01 89 07 75 97Appel gratuit
Du lundi au vendredi 08h30 - 20h00
hors jours fériés



01 89 07 75 97Appel gratuit du lundi au vendredi 08h30 - 19h00 hors jours fériés


                

                                
Obtenir un devis                                
                                
Me connecter                                

                
            
        
    



                    







﻿﻿Estimez gratuitement le coût de votre assurance professionnelle

Votre devis Responsabilité Civile Professionnelle

Votre contrat Responsabilité Civile Professionnelle



    .navbar-static-top .nav {
        padding: 15px 0;
    }

    @media (max-width: 767px) {
        .navbar-brand {
            padding: 0;
        }

            .navbar-brand img {
                margin-top: 5px;
                margin-left: 13px;
            }
    }





×





                    
                        Pourriez-vous remplir les informations suivantes ?
                    

                                
                    


 

                    


            

                
                


	



                
                








        var $questionForm = null,
            $quoteContinueButton = null,
            newLocation = &quot;&quot;,
            redirect = &quot;False&quot; == &quot;True&quot;;

        if (redirect) {
            newLocation = &quot;/Public/Quote?PackageId=12462&amp;amp;quoteRef=Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&amp;amp;quoteIsReadOnly=False&amp;amp;fromNavLink=False&quot;;
            window.open(newLocation, &quot;_self&quot;);
        }

        $(function () {
            &quot;use strict&quot;;

            $quoteContinueButton = $(&quot;#continueButton&quot;);

            $questionForm = $quoteContinueButton
                .parentsUntil(&quot;form&quot;)
                .parent();

            if ($quoteContinueButton.length !== 0 &amp;&amp; $questionForm.length !== 0) {

                $quoteContinueButton.on(&quot;click&quot;, null, function () {

                    $questionForm
                        .find(&quot;#goBackward&quot;)
                        .remove()
                        .end()
                        .submit();
                });
            }

            ShowHelpTextOnFocus(&quot;WhenClicked&quot;);

            $(document.body).on('change', '.form-control[data-val-regex-pattern], .form-control[data-val-regexci-pattern]', function () {
                var txtVal = $(this).val().trim();
                $(this).val(txtVal);
            });
        });




		
			
            



            
            
                



            


                    
                        Le preneur d'assurance
                    

            



            



        
            Votre nom
        
    
        




        

            
                
                    





                    
                        


                            
                            Madame
                        
                    
                    
                        


                            
                            Monsieur
                        
                    
            
                
            




    

       var func_Titre = (function(){

           var showOptionalText = false;

            if (showOptionalText) {

                var Titre = 'Titre';
                var dropdownadditionalchoiceinfo118957 = 'dropdownadditionalchoiceinfo118957';

                var extraInfoSelector = &quot;#&quot; + 'dropdownadditionalchoiceinfo118957';

                $(&quot;#&quot; + 'Titre').change(function(){
                    var lastchild = $(&quot;#&quot; + 'Titre' + &quot; option:last-child&quot;).val();
                    toggleAdditionInfoTextBox(this,lastchild);
                });

                $('.' + 'Titre').click(function (){
                    var lastchild = $(&quot;.&quot; + 'Titre' + &quot;_last&quot;).val();
                    toggleAdditionInfoTextBox(this,lastchild);
                });

                function toggleAdditionInfoTextBox(self,lastchildVal) {
                    var current = $(self).val();

                    if (lastchildVal === current) {
                        showAdditionInfoTextBox();
                    } else {
                        hideAdditionInfoTextBox();
                    }
                }

                function showAdditionInfoTextBox() {
                    appendToParent();
                    $(extraInfoSelector).show();
                    $(extraInfoSelector + &quot; INPUT&quot;).focus();
                }

                function hideAdditionInfoTextBox() {
                    appendToParent();
                    $(extraInfoSelector).hide();
                }

                function appendToParent() {
                    var parent = $(extraInfoSelector).parent();
                    $(parent).append($(extraInfoSelector));
                }

                return {
                    showAdditionInfoTextBox:showAdditionInfoTextBox,
                    hideAdditionInfoTextBox:hideAdditionInfoTextBox
                };
            }

           return {
               showAdditionInfoTextBox:{},
               hideAdditionInfoTextBox:{}
           };
       })();

        $(function () {
            var showOptionalText = false;
            var additionalInfoEntered = false;
            var lastQuestionWasSelected= false;
                    //or last value selected
            if (showOptionalText &amp;&amp; (additionalInfoEntered || lastQuestionWasSelected)) {
                func_Titre.showAdditionInfoTextBox();
            }
        });

        function autoCompleteShowAdditionInfo_Titre() {
            var showOptionalText = false;
            if (showOptionalText) {
                func_Titre.showAdditionInfoTextBox();
            }
        }

        function autoCompleteHideAdditionInfoTextBox_Titre() {
            var showOptionalText = false;
            if (showOptionalText) {
                func_Titre.hideAdditionInfoTextBox();
            }
        }

    

                
            

            
                
                    Nom
                
                

                    
                
      
        
        Nom
      
      
    
                
                    
                
            

            
                
                    Prénom
                
                

                    
                
      
        
        Prénom
      
      
    
                
                    
                
            

        

        

        
        
    
    
        
            
        
    
            



            



        
            Email professionnel
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Email professionnel
      
      
    
    
        
            
        
    
            



            



        
            Numéro de téléphone professionnel
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Numéro de téléphone professionnel
      
      
    
    
        
            
        
    
            
            
                



            


                    
                        Mon entreprise
                    

            



            



        
            Nom de l’entreprise
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Nom de l’entreprise
      
      
    
    
        
            
        
    
            



            



        
            Adresse de l’entreprise
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Adresse de l’entreprise
      
      
    
    
        
            
        
    
            



            



        
            Code postal de l'entreprise
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Code postal de l'entreprise
      
      
    
    
        
            
        
    
            



            



        
            Ville de l’entreprise
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Ville de l’entreprise
      
      
    
    
        
            
        
    
            



            








    


        

    

    
Je m’oppose à ce que mon numéro de téléphone et mon adresse email soient utilisés pour recevoir des offres commerciales d’Hiscox 




    


            
        

		

	
	
	
	
	
	


	

		
    Saved





	



                
                
                

		
			
				


    Saved


             Étape précédente






    var $questionForm = null,
        $quoteBackButton = null,
        $quoteGoBackwardElement = &quot;&lt;input type=\&quot;hidden\&quot; id=\&quot;goBackward\&quot; name=\&quot;goBackward\&quot; value=\&quot;false\&quot; data-ays-ignore=\&quot;true\&quot; />&quot;
        ;

    var backClickHandlerSet = false;

    $(function () {

        &quot;use strict&quot;;

        $quoteBackButton = $(&quot;#backButton&quot;);

        $questionForm = $quoteBackButton
            .parentsUntil(&quot;form&quot;)
            .parent();

        function clearIsDirty() {

            if ($questionForm.length !== 0) {

                $questionForm.remove(&quot;#goBackward&quot;);
            }
        }

        function setIsDirty () {

            if ($questionForm.length !== 0) {

                $questionForm
                    .append($quoteGoBackwardElement)
                    .find(&quot;#goBackward&quot;)
                    .val(true);
            }
        }

        if ($questionForm.length !== 0) {

            // Enable detection of changes to question answers
            $questionForm.areYouSure({ silent : true });
        }

        if ($quoteBackButton.length !== 0 &amp;&amp; $questionForm.length !== 0) {

            $questionForm.on(&quot;dirty.areYouSure&quot;, null, function () {

                setIsDirty();
                $questionForm.addClass(&quot;dirty&quot;);  // This should not be necessary, but there seems to be some unreliability when reading the property later
            });

            if (!backClickHandlerSet) {
                backClickHandlerSet = true;

                $quoteBackButton.on(&quot;click&quot;, null, function () {

                    var $questionForm = $(this)
                        .parentsUntil(&quot;form&quot;)
                        .parent();

                    $questionForm.trigger(&quot;checkform.areYouSure&quot;);

                    if ($questionForm.hasClass(&quot;dirty&quot;)) {
                        // Changes detected to the question answers.  Ensure client side validation is enforced.
                        //$questionForm.submit();
                        saveAnswersFrom(window.location.href, 'quoteRef', '/Public/QuickQuoteQuestions?PackageId=12462&amp;quoteRef=Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&amp;createdFromType=NewBusiness&amp;isUpdate=False&amp;pageNumber=2&amp;goBackwards=True&amp;releaseDateOption=NotSet&amp;showSendReferralEmailButton=False&amp;quoteIsReadOnly=False', 'SavedFrom', 'SaveButton');
                        // saveAnswers(window.location.href, 'quoteRef', '/Public/QuickQuoteQuestions?PackageId=12462&amp;quoteRef=Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&amp;createdFromType=NewBusiness&amp;isUpdate=False&amp;pageNumber=2&amp;goBackwards=True&amp;releaseDateOption=NotSet&amp;showSendReferralEmailButton=False&amp;quoteIsReadOnly=False');
                    } else {
                        // No changes were detected to the question answers.  Bypass client side validation and redirect to the previous page.
                        clearIsDirty();
                        location.href = &quot;/Public/QuickQuoteQuestions?PackageId=12462&amp;quoteRef=Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&amp;createdFromType=NewBusiness&amp;isUpdate=False&amp;pageNumber=2&amp;goBackwards=True&amp;releaseDateOption=NotSet&amp;showSendReferralEmailButton=False&amp;quoteIsReadOnly=False&quot;;
                    }
                });
            }
        }
    });


						

								Save
							Étape suivante 
						
				
			
		


                
                
                




                


    .toastr-btn {
        float: right;
        margin-right: 4px;
    }



    


    
        
            
                
                    Your session has expired, you will have to restart your quote
                
                
                    
                        Restart Quote
                    
                
            
        
    



    &quot;use strict&quot;;

    Date.prototype.getUTCTime = function () {
        return (this.getTime() + (this.getTimezoneOffset() * 60 * 60 * 1000)); // Add the offset hours in ms
    }

    Date.prototype.getUTC = function () {
        return new Date(this.getUTCTime());
    }

    Date.__proto__.utc = function () {
        return new Date().getUTCTime();
    }

    Date.__proto__.UTCNow = function () {
        return new Date(Date.utc());
    }

    const sessionWarningCookieName = &quot;__Host-SessionExpirationWarningTime&quot;;
    const minutesBeforeExpirationToWarn = 3;
    const minutesBeforeExpirationToWarnMili = minutesBeforeExpirationToWarn * 60000;
    const isEnabled = true;
    const sessionLengthMili = 900000;

    var sessionWarningTime = new Date();
    var sessionWarningTimer = 0;
    var sessionExpiredTimer = 0;

    var isLoginTypePublic = true

    $(function () {
        if (isEnabled === true) {
            InitialiseSessionAlerts();
        }
    });

    function SetSessionWarningTime(warningMilliseconds) {
        sessionWarningTime = new Date(Date.now() + warningMilliseconds);
    }

    function InitialiseSessionAlerts() {
        var milisecondsTillWarning = sessionLengthMili - minutesBeforeExpirationToWarnMili;

        if (!isLoginTypePublic || (window.location.href.includes(&quot;quoteRef&quot;) || window.location.href.includes(&quot;QuickQuoteQuestions&quot;) || window.location.href.includes(&quot;PreQuoteQuestionsContinue&quot;))) {
            ClearTimers();
            SetSessionWarningTime(milisecondsTillWarning);
            SetTimers(milisecondsTillWarning, sessionLengthMili);
        }
    }

    function AdjustSessionTimers(milisecondsTillWarning) {
        var milisecondsTillExpiration = milisecondsTillWarning + minutesBeforeExpirationToWarnMili;

        ClearTimers();
        SetSessionWarningTime(milisecondsTillWarning);
        SetTimers(milisecondsTillWarning, milisecondsTillExpiration);
    }

    function ClearTimers() {
        window.clearTimeout(sessionWarningTimer);
        window.clearTimeout(sessionExpiredTimer);
    }

    function SetTimers(warningWait, expirationWait) {
        if (warningWait > 0) {
            sessionWarningTimer = window.setTimeout(PromptKeepSessionActive, warningWait);
        }
        sessionExpiredTimer = window.setTimeout(AlertSessionExpired, expirationWait);

    }

    function CheckAndUpateTimers() {
        var validTimeChange = false;
        var dateString = getSessionExpiryFromCookie(sessionWarningCookieName);
        var expirationTime = new Date(dateString);
        var cookieWarningTime = new Date(expirationTime.getTime() - minutesBeforeExpirationToWarnMili);

        if (Date.now() &lt; expirationTime.getTime()) {
            if (cookieWarningTime.getTime() != sessionWarningTime.getTime()) {
                var timeDiff = Math.max(0, (cookieWarningTime.getTime() - Date.now()));

                // Cookie was changed by another tab, need to upate the timers in this tab
                if (timeDiff > 0) {
                    AdjustSessionTimers(timeDiff);
                    validTimeChange = true;
                }
            }
        }

        return validTimeChange;
    }

    function PromptKeepSessionActive() {
        toastr.remove();

        if (CheckAndUpateTimers()) {
            return;
        }

        var common = new Instanda.Common();
        if (isLoginTypePublic) {
            var message = 'Your session is about to expire, do you wish to extend it?';
            var buttonText = 'Yes';
        } else {
            var message = 'Your session is about to expire, do you wish to extend it?';
            var buttonText = 'Yes';
        }


        var encryptedQuoteRef = null;

             encryptedQuoteRef = 'Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7';

        toastr.warning(&quot;&lt;div>&lt;button type='button' id='confirmationButtonYes' class='btn-extend-session btn instanda-button btn btn-primary toastr-btn'>&quot; + buttonText + &quot;&lt;/button>&lt;/div>&quot;, message,
            {
                positionClass: &quot;toast-top-center&quot;,
                timeOut: 0,
                extendedTimeOut: 0,
                preventDuplicates: false,
                closeButton: true,
                allowHtml: true,
                escapeHtml: false,
                onShown: function (toast) {
                    $(&quot;#confirmationButtonYes&quot;).click(function () {
                        common.keepSessionActive(encryptedQuoteRef);
                        InitialiseSessionAlerts();
                        toastr.remove();
                    });
                }
            });
    }

    function AlertSessionExpired() {
        toastr.remove();

        if (CheckAndUpateTimers()) {
            return;
        }

        if (isLoginTypePublic) {
            if (true) {
                $('#publicLiveSessionExpiredPopupDialog').modal('show');
            } else {
                document.getElementById('publicLiveSessionExpiredPopupDialog').style.display = &quot;block&quot;;
            }
        } else {

            var message = 'Your session has expired, you will be redirected to the login page';
            var hideButtonText = 'Hide';
            var loginButtonText = 'Login';

            toastr.error(&quot;&lt;div>&lt;button type='button' id='confirmationButtonOK' class='btn instanda-button btn btn-primary toastr-btn'>&quot; + hideButtonText + &quot;&lt;/button>&quot; +
                &quot;&lt;button type='button' id='loginButtonOK' class='btn instanda-button btn btn-primary toastr-btn'>&quot; + loginButtonText + &quot;&lt;/button>&lt;/div>&quot;,
                message,
                {
                    positionClass: &quot;toast-top-center&quot;,
                    timeOut: 0,
                    extendedTimeOut: 0,
                    preventDuplicates: false,
                    closeButton: true,
                    allowHtml: true,
                    escapeHtml: false,
                    onShown: function (toast) {
                        $(&quot;#confirmationButtonOK&quot;).click(function () {
                            $(&quot;.toast-close-button&quot;).click();
                        });
                        $(&quot;#loginButtonOK&quot;).click(function () {
                            RedirectToLogin();
                        });
                    }
                });
        }

        killSession();
    }

    $(&quot;#publicLiveSessionExpiredRestartQuoteButton&quot;).click(function () {
        RestartQuote();
    });

    function RestartQuote() {
            window.location.href = &quot;/Public/QuickQuoteQuestions?packageId=12462&quot;;
    }

    function CreateCookie(name, value, days) {
        if (days) {
            var date = new Date();
            date.setTime(date.getTime() + (days * 24 * 60 * 60 * 1000));
            var expires = &quot;; expires=&quot; + date.toGMTString();
        } else {
            var expires = &quot;&quot;;
        }

        document.cookie = name + &quot;=&quot; + value + expires + &quot;; path=/&quot;;
    }

    function getSessionExpiryFromCookie(name) {
       const nameEQ = name + &quot;=&quot;;
        const ca = document.cookie.split(';');
        for (let i = 0; i &lt; ca.length; i++) {
            let c = ca[i];
            while (c.charAt(0) == ' ') c = c.substring(1, c.length);
            if (c.indexOf(nameEQ) == 0) {
                let value = c.substring(nameEQ.length, c.length);
                let expiry = new Date(value);
                return expiry;
            };
        }

        // If it can't find the cookie then return current time (i.e. act like the session just expired)
        return Date.UTCNow();
    }

    function RedirectToLogin() {
        var loginType = 'Public';

        if (loginType == &quot;Client&quot;) {
            window.location.href = &quot;/&quot;;
            return;
        }

        $.ajax({
            type: &quot;GET&quot;,
            url: &quot;/Public/GetSessionLoginUrl?login=&quot; + loginType,
            success: function (data) {
                window.location.href = data;
                return;
            },
            error: function (jqXHR, textStatus, error) {

                var errorMessage = &quot;&quot;;

                if (jqXHR.responseJSON) {
                    if (jqXHR.responseJSON.IsRedirect) {
                        $('form').trigger('reinitialize.areYouSure');
                        window.location.href = jqXHR.responseJSON.RedirectURL;
                        return;
                    } else if (jqXHR.responseJSON.HTMLContent) {
                        $('form').trigger('reinitialize.areYouSure');
                        var htmlPageContent = jqXHR.responseJSON.HTMLContent;
                        document.write(htmlPageContent);
                        document.close();
                        return;
                    } else if (jqXHR.responseJSON.ErrorMessage) {
                        errorMessage = jqXHR.responseJSON.ErrorMessage;
                    } else {
                        errorMessage = jqXHR.responseJSON;
                    }
                } else {
                    errorMessage = JSON.parse(jqXHR.responseText);
                }
            }
        });
    }

    function killSession() {
        $.ajax({
            type: &quot;GET&quot;,
            url: &quot;/Public/AjaxPublicKillSession&quot;,
            success: function (data) { },
            error: function (jqXHR, textStatus, error) {

                var errorMessage = &quot;&quot;;

                if (jqXHR.responseJSON) {
                    if (jqXHR.responseJSON.IsRedirect) {
                       //the user will be redirected later based on the session expiration dialog choice
                        return;
                    } else if (jqXHR.responseJSON.HTMLContent) {
                        $('form').trigger('reinitialize.areYouSure');
                        var htmlPageContent = jqXHR.responseJSON.HTMLContent;
                        document.write(htmlPageContent);
                        document.close();
                        return;
                    } else if (jqXHR.responseJSON.ErrorMessage) {
                        errorMessage = jqXHR.responseJSON.ErrorMessage;
                    } else {
                        errorMessage = jqXHR.responseJSON;
                    }
                } else if (jqXHR.status !== 403) {
                    errorMessage = jqXHR.responseText;
                }

                if (errorMessage !== '') {
                    throw (errorMessage);
                }
            }
        });
    }

    $(document).ajaxSend(function (e, xhr, options) {

        var cookieTimeMili = new Date(getSessionExpiryFromCookie(sessionWarningCookieName)).getTime();
        var expiryTimeMili = cookieTimeMili + minutesBeforeExpirationToWarnMili;
        var currentTimeMili = new Date().getTime();

        var url = options.url;
        if (url != null &amp;&amp; url === &quot;/Public/AjaxPublicKillSession&quot;) {
            return;
        }

        if (expiryTimeMili &lt; currentTimeMili) {
            xhr.abort();
            $('form').trigger('reinitialize.areYouSure');
            window.location.reload();
        }
    });

    $(document).ajaxComplete(function (event, jqXHR, ajaxOptions) {

        var extendSession = true;

        if (typeof jqXHR.statusText !== &quot;undefined&quot; &amp;&amp; jqXHR.statusText == &quot;abort&quot; &amp;&amp; typeof jqXHR.status !== &quot;undefined&quot; &amp;&amp; jqXHR.status == 0) {
            extendSession = false;
        } else if (typeof ajaxOptions.url !== &quot;undefined&quot; &amp;&amp; ajaxOptions.url === &quot;/Public/AjaxPublicKillSession&quot;) {
            extendSession = false;
        } else if (typeof jqXHR.responseJSON !== &quot;undefined&quot; &amp;&amp; typeof jqXHR.responseJSON.notExtendSession !== &quot;undefined&quot; &amp;&amp; typeof jqXHR.responseJSON.notExtendSession === &quot;boolean&quot;) {
            extendSession = !jqXHR.responseJSON.notExtendSession;
        }

        if (extendSession) {
            InitialiseSessionAlerts();
            //we only want to clear toast warnings
            $('.btn-extend-session').each(function (i, obj) {
                toastr.remove();
            });
        }
    });


            

            






Votre devis






Pour votre activité


Responsabilité Civile Professionnelle



Protection Juridique





Prime TTC 14,39 €






























Total TTC

14,39
par mois






Changer mon offre






€163,65TTC



€83,33TTC

soit 166,65 € par an



€42,41TTC

soit 169,64 € par an



€14,39TTC

soit 172,64 € par an

            
            




Les données personnelles fournies seront utilisées pour vous adresser le devis que vous sollicitez et pour la gestion de votre contrat d'assurance. Hiscox traite vos données personnelles avec le plus grand soin et ne les vend pas à des tiers à des fins de marketing. En cliquant sur &quot;Étape suivante&quot;, je déclare avoir pris connaissance de la notice d'information relative au traitement de mes données personnelles 










 

            
            
        



        
             
        
        

        

            $(function () {
                PreventDoubleSubmission($('form'));

            });

            window.thousandsSeperator = '&amp;#160;';
            window.decimalSeperator = ',';

            var validator = function () {

                var pub = {};

                pub.run = function (form) {
                    form.removeData('validator');
                    form.removeData('unobtrusiveValidation');
                    form.removeData('submitted');

                    $.validator.unobtrusive.parse(form);
                };

                return pub;
            }();

            $(&quot;form&quot;).submit(function (e) {

                var form = $(this);

                validator.run(form);
            });


        






  /* System hack */
  (function() {
    // hack system createAutoCompleteQuestion()
    const oldCreateAutoCompleteQuestion = createAutoCompleteQuestion;
    window.createAutoCompleteQuestion = function() {
      oldCreateAutoCompleteQuestion.apply(this, arguments);

      const questionId = arguments[0];
      const target = $(&quot;#&quot; + questionId).closest('.instanda-question-input')[0];
      const observer = new MutationObserver(function(mutationList, observer) {
        for (const mutation of mutationList) {
          if (mutation.type == 'childList' &amp;&amp; mutation.addedNodes.length > 0 &amp;&amp; mutation.addedNodes[0].classList.contains('typeahead')) {
            $(mutation.target).trigger('autoComplete.built');
          }
        }
      });
      observer.observe(target, {childList: true});
    }
  })();


  /* set tippy theme */
  tippy.setDefaultProps({
    'theme': 'custom'
  });


  /* Add class to body for style to fix input zoom for specific device or browser */
  navigator.userAgent.match(/iPhone|iPad|iPod/i) &amp;&amp; document.body.classList.add('iOS');
  navigator.userAgent.match(/Safari/i) &amp;&amp; document.body.classList.add('safari');


  /**
   * DOM Helpers - define html attributes for relocating elements
   *
   * Attribute names:
   *   - data-append-to
   *   - data-prepend-to
   *   - data-insert-after
   *   - data-insert-before
   *
   * Attribute values:
   *   valid jquery selector string
   */
  (function() {
    function domHelpers() {
      $('[data-append-to]').once('append-to').each(function() {
        var selector = $(this).data('append-to');
        $(this).appendTo(selector);
      });
      $('[data-prepend-to]').once('prepend-to').each(function() {
        var selector = $(this).data('prepend-to');
        $(this).prependTo(selector);
      });
      $('[data-insert-after]').once('insert-after').each(function() {
        var selector = $(this).data('insert-after');
        $(this).insertAfter(selector);
      });
      $('[data-insert-before]').once('insert-before').each(function() {
        var selector = $(this).data('insert-before');
        $(this).insertBefore(selector);
      });
      $('[data-toggle-by-variable]').once('toggle-by-variable').each(function() {
        var name = $(this).data('toggle-by-variable');
        $(this).toggle(Boolean(Instanda.Variables[name]));
      });
    }

    $(document)
      .ready(domHelpers)
      .ajaxComplete(domHelpers);
  })();


  /**
   * Util functions under namespace `Instanda.utils`
   *
   * isMobile()
   *   check if it is mobile
   */
  (function() {
    Instanda.utils = Instanda.utils || {};

    Instanda.utils.isMobile = function() {
      return window.outerWidth &lt;= 767;
    }
  })();


  // example: https://hiscoxdesign.instanda.com/Public/QuickQuoteQuestions?PackageId=12462&amp;pageNumber=1&amp;Sector_CHOICE=V%C3%A9hicule%20de%20Tourisme%20avec%20Chauffeur&amp;Turnover_NUM=2000&amp;Activity_CHOICE=Je%20ne%20trouve%20pas%20mon%20activit%C3%A9#debug
  (function() {
    $(window).one(&quot;load&quot;, function () {
      setTimeout(function() {
        const search = new URLSearchParams(window.location.search);
        for (const key of search.keys()) {
          const control = document.getElementById(key);
          if (control &amp;&amp; control.type !== 'hidden') {
            const value = search.get(key);
            $(control).val(value).trigger('change');

            if (control.tagName === 'SELECT') {
              $(control).closest('.instanda-question-input').one('autoComplete.built', function(event) {
                const $input = $(event.currentTarget).find('.tt-input');
                const controlId = $input.attr('id');
                const value = search.get(controlId);
                $(event.currentTarget.children[0]).find('.tt-input').typeahead('val', value);
              });
            }
          }
        }
      }, 0);
    });
  }) ();


  /**
   * MDC Builders
   */
  (function() {
    // Templates
    var mdcSelectTempl = document.getElementById(&quot;mdc-select-tmpl&quot;).innerHTML;
    var mdcTextfieldTempl = document.getElementById(&quot;mdc-textfield-tmpl&quot;).innerHTML;
    var mdcSwitchTempl = document.getElementById(&quot;mdc-switch-tmpl&quot;).innerHTML;
    var mdcMenuTempl = document.getElementById(&quot;mdc-menu-tmpl&quot;).innerHTML;

    Mustache.parse(mdcSelectTempl);
    Mustache.parse(mdcTextfieldTempl);
    Mustache.parse(mdcSwitchTempl);
    Mustache.parse(mdcMenuTempl);

    // DEBUG
    function createDebugSwitch() {
      var rendered = Mustache.render(mdcSwitchTempl, {});
      var elem = $(rendered)[0];
      elem.style.cssText=&quot;position:fixed; bottom:15px; right:15px; display:block !important;&quot;;
      document.body.append(elem);

      var component = mdc.switchControl.MDCSwitch.attachTo(elem.querySelector('.mdc-switch'));
      component.nativeControl_.addEventListener('change', function(event) {
        $('body').toggleClass('my-mdc-off', event.target.checked);
      });
    }
    $(document).ready(function() {
      window.location.hash.indexOf('debug') > -1 &amp;&amp; createDebugSwitch();
    });

    // Classes
    class MDCComponentBuilder {
      constructor(ctrlElem, labelText, ctrlContainer, labelContainer, helpContainer = null) {
        this.ctrlElem = ctrlElem;
        this.labelText = this.htmlDecode(labelText);
        this.ctrlContainer = ctrlContainer;
        this.labelContainer = labelContainer;
        this.helpContainer = helpContainer;
        this.template = '';
        this.mutatedAttributes = ['hidden'];
      }

      htmlDecode(input) {
        var doc = new DOMParser().parseFromString(input, &quot;text/html&quot;);
        return doc.documentElement.textContent;
      }

      init() {
        this.mdcWrapperElement = null;
        this.mdcComponent = null;

        // @deprecated
        this.helpText = this.getHelpText();
      }

      // @deprecated
      getHelpText() {
        var $dom = $(this.ctrlContainer).find('.field-validation-error');
        return $dom.length > 0 ? $dom.html() : &quot;&quot;;
      }

      hideElements() {
        $(this.ctrlContainer).addClass('my-mdc-hide').attr('aria-hidden', &quot;true&quot;);
        $(this.ctrlElem).attr('tabindex', '-1');
        this.labelContainer &amp;&amp; $(this.labelContainer).addClass('my-mdc-hide').attr('aria-hidden', &quot;true&quot;);
      }

      isBuilt() {
        return true &amp;&amp; $(this.ctrlContainer).data('builder-processed');
      }

      build() {
        if (this.isBuilt()) return;

        this.prepareComponent();
        this.moveHelpIcon();
        this.syncStyle();
        this.bindEvents();
        this.observeMutation();
        this.hideElements();
        this.insert();
        $(this.ctrlContainer).data('builder-processed', true);
        $(this.mdcWrapperElement).data('builder-processed', true);
      }

      prepareComponent() {
      }

      moveHelpIcon() {
        const $iconContainer = $(this.ctrlContainer).find('.instanda-responsive-help-icon-container');
        $iconContainer.length > 0 &amp;&amp; $(this.mdcWrapperElement).append($iconContainer);
      }

      syncStyle() {
        this.mdcWrapperElement.setAttribute('style', this.ctrlElem.getAttribute('style') || '');
      }

      bindEvents() {
      }

      observeMutation() {
        var config = { childList: true, attributes: true, subtree: true };
        this.observer = new MutationObserver(function(mutationRecords, observer) {
          var syncStyle = false;
          var rebuild = false;
          mutationRecords.forEach(function(mutationRecord) {
            if (mutationRecord.type == 'attributes') {
              if (mutationRecord.attributeName == 'style') syncStyle = true;
              if (this.mutatedAttributes.indexOf(mutationRecord.attributeName) > 0) rebuild = true;
            }
            if (mutationRecord.type == 'childList') {
              rebuild = true;
            }
          }.bind(this));

          if (syncStyle) this.syncStyle();
          if (rebuild) this.rebuild();
        }.bind(this))
        this.ctrlElem &amp;&amp; this.observer.observe(this.ctrlElem, config);
        this.helpContainer &amp;&amp; this.observer.observe(this.helpContainer, config);
      }

      destroy() {
        this.observer &amp;&amp; this.observer.disconnect();
        this.mdcComponent &amp;&amp; this.mdcComponent.destroy();
        $(this.mdcWrapperElement).removeData('builder-processed').remove();
        $(this.ctrlContainer).removeData('builder-processed');
      }

      revert() {
        this.destroy();
        $(this.ctrlContainer).removeClass('my-mdc-hide').removeAttr('aria-hidden');
        $(this.ctrlElem).removeAttr('tabindex');
        this.labelContainer &amp;&amp; $(this.labelContainer).removeClass('my-mdc-hide').removeAttr('aria-hidden', &quot;true&quot;);
      }

      rebuild() {
        this.destroy();
        this.init();
        this.build();
      }

      insert() {
        $(this.ctrlContainer).after(this.mdcWrapperElement);
      }
    }

    class MDCSwitchBuilder {
      constructor(ctrlElems, labelText, ctrlContainer, labelContainer, valueOn=&quot;Yes&quot;, valueOff=&quot;No&quot;) {
        this.ctrlElems = ctrlElems;
        this.labelText = labelText;
        this.ctrlContainer = ctrlContainer;
        this.labelContainer = labelContainer;
        // this.valueOn = valueOn;
        // this.valueOff = valueOff;
        this.ctrlElemOn = $(this.ctrlElems).filter(`[value=&quot;${valueOn}&quot;]`)[0];
        this.ctrlElemOff = $(this.ctrlElems).filter(`[value=&quot;${valueOff}&quot;]`)[0];

        this.template = mdcSwitchTempl;
        this.init();
      }

      init() {
        this.mdcWrapperElement = null;
        this.mdcComponent = null;
        this.observer = null;
      }

      hideElements() {
        $(this.ctrlContainer).addClass('my-mdc-hide').attr('aria-hidden', &quot;true&quot;);
        $(this.ctrlElems).attr('tabindex', '-1');
        this.labelContainer &amp;&amp; $(this.labelContainer).addClass('my-mdc-hide').attr('aria-hidden', &quot;true&quot;);
      }

      isBuilt() {
        return true &amp;&amp; $(this.ctrlContainer).data('builder-processed');
      }

      build() {
        if (this.isBuilt()) return;

        this.prepareComponent();
        this.bindEvents();
        this.observeMutation();
        this.hideElements();
        this.insert();
        $(this.ctrlContainer).data('builder-processed', true);
        $(this.mdcWrapperElement).data('builder-processed', true);
      }

      prepareComponent() {
        var data = {}
        data.label = this.labelText;
        data.disabled = (
          !this.ctrlElemOn.checked &amp;&amp; this.ctrlElemOn.disabled
        ) || (
          !this.ctrlElemOff.checked &amp;&amp; this.ctrlElemOff.disabled
        );
        data.checked = this.isChecked();
        var rendered = Mustache.render(this.template, data);
        this.mdcWrapperElement = $(rendered)[0];
        this.mdcComponent = mdc.switchControl.MDCSwitch.attachTo(this.mdcWrapperElement.querySelector('.mdc-switch'));
      }

      isChecked() {
        return $(this.ctrlElemOn).prop('checked');
      }

      bindEvents() {
        var builder = this;

        this.mdcComponent.nativeControl_.addEventListener('change', this._changeHandler.bind(this));

        $(this.ctrlElemOn)
          .on('click.mdc', function(event) {
            builder.mdcComponent.foundation_.setChecked(true);
          })

        $(this.ctrlElemOff)
          .on('click.mdc', function(event) {
            builder.mdcComponent.foundation_.setChecked(false);
          })
      }

      observeMutation() {
        var config = { childList: true, attributes: true, subtree: true };
        this.observer = new MutationObserver(function(mutationRecords, observer) {
          // var syncStyle = false;
          var rebuild = false;
          mutationRecords.forEach(function(mutationRecord) {
            if (mutationRecord.type == 'attributes') {
              // if (mutationRecord.attributeName != 'style') syncStyle = true;
              if (mutationRecord.attributeName == 'hidden') rebuild = true;
              if (mutationRecord.attributeName == 'disabled') rebuild = true;
            }
            if (mutationRecord.type == 'childList') {
              rebuild = true;
            }
          });

          // if (syncStyle) this.syncStyle();
          if (rebuild) this.rebuild();
        }.bind(this))
        this.ctrlElemOn &amp;&amp; this.observer.observe(this.ctrlElemOn, config);
        this.ctrlElemOff &amp;&amp; this.observer.observe(this.ctrlElemOff, config);
      }

      insert() {
        $(this.ctrlContainer).after(this.mdcWrapperElement);
      }

      destroy() {
        this.mdcComponent.nativeControl_.removeEventListener('change', this._changeHandler.bind(this));
        $(this.ctrlElemOn).off('click.mdc');
        $(this.ctrlElemOff).off('click.mdc');
        this.mdcComponent.destroy();
        $(this.mdcWrapperElement).removeData('builder-processed').remove();
        $(this.ctrlContainer).removeData('builder-processed');
      }

      rebuild() {
        this.destroy();
        this.init();
        this.build();
      }

      _changeHandler(event) {
        if (event.target.checked) {
          $(this.ctrlElemOn).click();
        } else {
          $(this.ctrlElemOff).click();
        }
      }
    }

    class MDCTextFieldBuilder extends MDCComponentBuilder {
      constructor(ctrlElem, labelText, ctrlContainer, labelContainer, helpContainer) {
        super(ctrlElem, labelText, ctrlContainer, labelContainer, helpContainer);

        this.template = mdcTextfieldTempl;
        this.mutatedAttributes = ['hidden', 'autocomplete'];
        this.init();
      }

      prepareComponent() {
        var data = {}
        data.id = this.ctrlElem.id ? this.ctrlElem.id + '--mdc' : null;
        data.label = this.labelText;
        data.disabled = this.ctrlElem.disabled;
        data.readOnly = this.ctrlElem.readOnly;
        data.pattern = this.ctrlElem.getAttribute('pattern');
        data.autocomplete = this.ctrlElem.getAttribute('autocomplete');
        data.value = this.ctrlElem.value;
        data.help = this.helpText;
        var rendered = Mustache.render(this.template, data);
        this.mdcWrapperElement = $(rendered)[0];
        this.mdcComponent = mdc.textField.MDCTextField.attachTo(this.mdcWrapperElement.querySelector('.mdc-text-field'));
      }

      bindEvents() {
        var builder = this;
        var $input = $(this.ctrlElem);
        var $mdcInput = $(&quot;input&quot;, this.mdcWrapperElement)

        $mdcInput
          .on(&quot;input.mdc&quot;, function(event) {
            $input.val(event.target.value).trigger('input');
          })
          .on(&quot;blur.mdc&quot;, function(event) {
            $input.trigger('blur');
          })
          .on(&quot;change.mdc&quot;, function(event) {
            $input.trigger('change');
          });

        $input
          .on(&quot;change.mdc&quot;, function(event) {
            builder.rebuild();
          })
          .on(&quot;mdc:rebuild&quot;, function(event) {
            builder.rebuild();
          })
      }

      destroy() {
        var $input = $(this.ctrlElem);
        var $mdcInput = $(&quot;input&quot;, this.mdcWrapperElement)

        $mdcInput
          .off(&quot;input.mdc&quot;)
          .off(&quot;blur.mdc&quot;);

        $input.off(&quot;change.mdc&quot;);

        super.destroy();
      }
    }

    class MDCSelectBuilder extends MDCComponentBuilder {
      constructor(ctrlElem, labelText, ctrlContainer, labelContainer) {
        super(ctrlElem, labelText, ctrlContainer, labelContainer);

        this.template = mdcSelectTempl;
        this.init();
      }

      init() {
        super.init();

        this.observer = null;
      }

      prepareComponent() {
        var data = {};
        data.id = this.ctrlElem.id ? this.ctrlElem.id + '--mdc' : null;
        data.label = this.labelText;
        data.selectedText = this.ctrlElem.selectedOptions[0] &amp;&amp; this.ctrlElem.selectedOptions[0].value;
        data.options = this.extractOptions();
        var rendered = Mustache.render(this.template, data);
        this.mdcWrapperElement = $(rendered)[0];
        this.mdcComponent = mdc.select.MDCSelect.attachTo(this.mdcWrapperElement.querySelector('.mdc-select'));
      }

      bindEvents() {
        var builder = this;
        var $select = $(this.ctrlElem);

        this.mdcComponent.listen(&quot;MDCSelect:change&quot;, function(event) {
          builder.ctrlElem.selectedIndex = event.detail.index;
          builder.ctrlElem.dispatchEvent(new Event('change'));
        })

        $select
          .on(&quot;change.mdc&quot;, function(event) {
            builder.rebuild();
          });
      }

      extractOptions() {
        var options = [];
        $(this.ctrlElem).children('option').each(function(index, optionElem) {
          var data = {};
          data.selected = $(optionElem).is(':selected');
          data.disabled = optionElem.disabled;
          data.hidden = optionElem.hidden;
          data.value = optionElem.value;
          data.text = data.value == '' ? '' : $(optionElem).html();
          options.push(data);
        });
        return options;
      }
    }

    class mdcTypeAheadBuilder {
      constructor(ctrlElem, menuElem, labelText, ctrlContainer, labelContainer) {
        this.ctrlElem = ctrlElem;
        this.menuElem = menuElem;
        this.labelText = labelText;
        this.ctrlContainer = ctrlContainer;
        this.labelContainer = labelContainer;
      }

      prepareComponent() {
        var rendered = Mustache.render(mdcTextfieldTempl, {label: this.labelText});
        this.mdcWrapperElement = $(rendered)[0];
        this.mdcComponent = mdc.textField.MDCTextField.attachTo(this.mdcWrapperElement.querySelector('.mdc-text-field'))

        var menuRendered = Mustache.render(mdcMenuTempl, {});
        this.mdcMenuElement =
        this.mdcMenuComponent = mdc.menu.MDCMenu.attachTo();
      }

      build() {
      }

      insert() {
      }
    }

    // building MDC component.
    var buildMDC = function(scope = document) {

      $('.questionItem').find('.instanda-question-hierarchy > select', scope).each(function(index, selectElem) {
        var $selectContainer = $(selectElem).parent();
        var $labelContainer = $selectContainer.prev('.instanda-question-label');
        if ($labelContainer.length === 0) {
          var $labelContainer = $selectContainer.closest('.form-group').prev('.instanda-question-label');
        }
        var labelText = $labelContainer.children('label').html();
        var builder = new MDCSelectBuilder(selectElem, labelText, $selectContainer[0], $labelContainer[0]);
        builder.build();

        var observerConfig = { childList: true };
        var observer = new MutationObserver(function(mutationRecords, observer) {
          mutationRecords.forEach(function(mutationRecord) {
            if (mutationRecord.type == 'childList'
            &amp;&amp; mutationRecord.removedNodes[0]
            &amp;&amp; mutationRecord.removedNodes[0].tagName == 'SELECT') {
              builder.revert();
              builder = null;
            }
          });
        });
        $selectContainer &amp;&amp; observer.observe($selectContainer[0], observerConfig);
      });

      $('.questionItem')
        .not('#question121029')
        .find('.form-group > .instanda-question-input', scope)
        .children('input[type=&quot;text&quot;], input[type=&quot;number&quot;], input[type=&quot;email&quot;], select')
        .each(function(index, ctrlElem) {
        var $inputContainer = $(ctrlElem).parent();
        var $labelContainer = $inputContainer.siblings('.instanda-question-inner-label');
        if ($labelContainer.length == 0) {
          $labelContainer = $inputContainer.closest('.questionItem > .instanda-text-question').prev();
        }
        var labelText =
          ctrlElem.name == 'AddressLine1' ? 'Adresse du local' :
          ctrlElem.name == 'AddressLine2' ? 'Complément d’adresse' :
          ctrlElem.name == 'Postcode' ? 'Code postal' :
          $labelContainer.children('label').html();
        var $helpContainer = $inputContainer.closest('.questionItem').find('[data-valmsg-for=&quot;'+ctrlElem.id+'&quot;]');

        if ($(ctrlElem).is('input[type=&quot;text&quot;], input[type=&quot;number&quot;], input[type=&quot;email&quot;]')) {
          var builder = new MDCTextFieldBuilder(ctrlElem, labelText, $inputContainer[0], $labelContainer[0], $helpContainer[0]);
        }
        if ($(ctrlElem).is('select')) {
          var builder = new MDCSelectBuilder(ctrlElem, labelText, $inputContainer[0], $labelContainer[0]);
        }
        builder.build();
      });

      $('.questionItem').find('.instanda-number-input, .instanda-text-input', scope).children('input[type=&quot;text&quot;], input[type=&quot;number&quot;], input[type=&quot;email&quot;]').each(function(index, ctrlElem) {
        var $inputContainer = $(ctrlElem).closest('.questionItem > .instanda-text-question');
        var $labelContainer = $inputContainer.prev();
        var labelText = $labelContainer.children('label').html();
        var $helpContainer = $(ctrlElem).closest('.questionItem').find('[data-valmsg-for=&quot;'+ctrlElem.id+'&quot;]');

        var builder = new MDCTextFieldBuilder(ctrlElem, labelText, $inputContainer[0], $labelContainer[0], $helpContainer[0]);
        builder.build();
      });

      $('.questionItem').
      not('#question119074').
      find('.instanda-question-parent-yes-no', scope).each(function(index, question) {
        var $ctrlContainer = $(question).find('> .instanda-text-question');
        var $labelContainer = $ctrlContainer.prev();
        var labelText = $labelContainer.children('label').html();
        var $ctrlElems = $ctrlContainer.find('.radio-inline > input[type=&quot;radio&quot;]');
        // convert to switch control if 2 radios input found.
        if ($ctrlElems.length == 2) {
          var builder = new MDCSwitchBuilder($ctrlElems, labelText, $ctrlContainer[0], $labelContainer[0]);
          builder.build();
        }
      });

      var selectors = [
        '#question119728',
        '#question119732',
        '#question119735',
        '#question119738'
      ];
      $(selectors.join(',')).each(function(index, coverElem) {
        // &lt;div id=&quot;&quot;>
        //  &lt;div class=&quot;instanda-questionHeader&quot;>&lt;h3>&lt;div>label&lt;/div>&lt;/h3>&lt;/div>
        //  &lt;div>
        //    &lt;input type=&quot;radio&quot; value=&quot;${on}&quot;>
        //    &lt;input type=&quot;radio&quot; value=&quot;${off}&quot;>
        //  &lt;/div>
        // &lt;/div>
        var $labelContainer = $(coverElem).children('.instanda-questionHeader')
        var $ctrlContainer = $labelContainer.next();
        var labelText = $labelContainer.find('h3 > div').html();
        var $ctrlElems = $ctrlContainer.find('input[type=&quot;radio&quot;]');
        var builder = new MDCSwitchBuilder($ctrlElems, labelText, $ctrlContainer[0], $labelContainer[0], $ctrlElems[0].value, $ctrlElems[1].value);
        builder.build();
      });

      // $('div').find('input[type=&quot;text&quot;], input[type=&quot;email&quot;]').not('[name*=&quot;OtherAnswer&quot;]').not('.focus-class-processed').each(function() {
      //   var $input = $(this);

      //   $input.addClass('focus-class-processed');

      //   // -- HTML structure scenario 1
      //   var $label = $input.closest('.instanda-question-input').siblings('div[class*=&quot;-label&quot;]');

      //   // -- HTML structure scenario 2
      //   if ($label.length == 0) {
      //     $label = $input.parent().parent().parent('.instanda-text-question.row').siblings('div[class*=&quot;-label&quot;]')
      //   }

      // // -- HTML structure scenario 3
      //   if ($label.length == 0) {
      //     $label = $input.closest('.instanda-text-question.row').siblings('div[class*=&quot;-label&quot;]');
      //   }

      //   if ($label.length > 0) {
      //     processTextInput($input, $label);
      //   }
      // })
    }

    // hack system functions
    var oldAddMultiItemByClone = addMultiItemByClone;
    window.addMultiItemByClone = function() {
      oldAddMultiItemByClone.apply(this, arguments);

      var here = arguments[1];

      if (arguments[7] == 'Activity_MI') {
        // remove all wrapper have not been processed by builder.
        $('#innerBody .mdc-component-wrapper').each(function(index, wrapperElem) {
          if (!$(wrapperElem).data('builder-processed')) {
            $(wrapperElem).remove();
          }
        })

        processItem();
      }

      // build the mdc for controls have not been processed by builder.
      $(document)
        .ready(buildMDC)
        .ajaxComplete(buildMDC);
    }

    function processItem() {
      $('#insertActivity_MIMultiItemsHere > .instanda-multi-item').each(function() {
        var thisItem = this;
        var clickHandler = function(event) {
          if (event.target.value == &quot;Yes&quot;) {
            $(thisItem).addClass('manual-selected');
          }
          if (event.target.value == &quot;No&quot;) {
            $(thisItem).removeClass('manual-selected');
          }
        };
        $(thisItem).off('click', '.radio-inline > input[type=&quot;radio&quot;]', clickHandler);
        $(thisItem).on('click', '.radio-inline > input[type=&quot;radio&quot;]', clickHandler);
      });
    }

    Instanda.mdc = Instanda.mdc || {};
    Instanda.mdc.MDCComponentBuilder = MDCComponentBuilder;
    Instanda.mdc.MDCTextFieldBuilder = MDCTextFieldBuilder;
    Instanda.mdc.MDCSelectBuilder = MDCSelectBuilder;
    Instanda.mdc.MDCSwitchBuilder = MDCSwitchBuilder;
    Instanda.mdc.build = buildMDC;
  })();


  /**
   * MDC Builders
   *
   * These builders will convert ordinary Instanda questions to material design component (MDC).
   * MDC script and HTML templates are loaded in &lt;head>.
   *
   * The interactions on MDC will sync to the cooresponding Instanda questions for system functions.
   * The builder will hide those Instanda questions after MDC built.
   *
   * Add `#debug` to the url will enable the toggle button at bottom right, it can toggle the Instanda questions
   * and MDC for debugging.
   *
   * This snippet provides a initial function `Instanda.mdc.build()`
   */
  (function() {
    // do nothing if there is no mdc builders
    if (!Instanda.mdc) return;

    $(document)
      .ready(Instanda.mdc.build)
      .ajaxComplete(function() { Instanda.mdc.build(document); });
  })();


  /**
   * Multi-step form
   *
   * It is a factory object for spliting form into multiple steps in one page
   */
  (function() {
    var multiStepForm = function(elem) {
      this._root = this._getElemFromArgument(elem);
      this._current = 0;
      this._forms = [];
    }
    multiStepForm.prototype.setBackBtn = function(elem) {
      var e = this._getElemFromArgument(elem);
      this.backBtn = e;
    };
    multiStepForm.prototype.setContinueBtn = function(elem) {
      var e = this._getElemFromArgument(elem);
      this.continueBtn = e;
    };
    multiStepForm.prototype.setPreviousBtn = function(elem, reference, method = 'appendTo') {
      var e = this._getElemFromArgument(elem);
      var ref = this._getElemFromArgument(reference);
      this.previousBtn = e;
      this.previousBtn &amp;&amp; ref &amp;&amp; this.previousBtn[method](ref);
    };
    multiStepForm.prototype.setNextBtn = function(elem, reference, method = 'appendTo') {
      var e = this._getElemFromArgument(elem);
      var ref = this._getElemFromArgument(reference);
      this.nextBtn = e;
      this.nextBtn &amp;&amp; ref &amp;&amp; this.nextBtn[method](ref);
    };
    multiStepForm.prototype.addForm = function(elem) {
      var e = this._getElemFromArgument(elem);
      e.length == 0 &amp;&amp; typeof elem == 'string' &amp;&amp; console.warn('MultiStepForm: cannot find form with selector ' + elem);
      return e.length > 0 &amp;&amp; this._forms.push(e) || false;
    };
    multiStepForm.prototype.start = function() {
      // do nothing when no forms defined.
      if (this._forms.length == 0) {
        return;
      }

      var self = this;

      // display first form by default
      $.each(this._forms, function(index, form) {
        if (index > 0) {
          self._hideElem(form);
        }
      });

      self._bindEvents();
      self._displayButtons();
    };
    multiStepForm.prototype._getElemFromArgument = function(elem) {
      if (typeof elem === 'string') {
        return $(elem);
      }
      if (elem instanceof jQuery) {
        return elem;
      }
      return null;
    }
    multiStepForm.prototype._hideElem = function(elem) {
      var e = this._getElemFromArgument(elem);
      e &amp;&amp; e.hide();
    };
    multiStepForm.prototype._showElem = function(elem) {
      var e = this._getElemFromArgument(elem);
      e &amp;&amp; e.show();
    };
    multiStepForm.prototype._bindEvents = function() {
      var self = this;
      self.nextBtn.on('click', function (e) { self._gotoNextForm() });
      self.previousBtn.on('click', function (e) { self._gotoPreviousForm() });
    };
    multiStepForm.prototype._gotoNextForm = function() {
      if (!this._isLastStep()) {
        this._hideElem(this._forms[this._current]);
        this._next();
        this._showElem(this._forms[this._current]);
        this._displayButtons();
      }
    };
    multiStepForm.prototype._gotoPreviousForm = function() {
      if (!this._isFirstStep()) {
        this._hideElem(this._forms[this._current]);
        this._previous();
        this._showElem(this._forms[this._current]);
        this._displayButtons();
      }
    };
    multiStepForm.prototype._isFirstStep = function() {
      return this._current == 0;
    };
    multiStepForm.prototype._isLastStep = function() {
      return this._current == this._forms.length - 1;
    };
    multiStepForm.prototype._next = function() {
      this._current++;
    };
    multiStepForm.prototype._previous = function() {
      this._current--;
    };
    multiStepForm.prototype._displayButtons = function() {
      if (this._isFirstStep()) {
        this._hideElem(this.previousBtn);
        this._showElem(this.backBtn);
      }
      if (!this._isFirstStep()) {
        this._showElem(this.previousBtn);
        this._hideElem(this.backBtn);
      }
      if (this._isLastStep()) {
        this._hideElem(this.nextBtn);
        this._showElem(this.continueBtn);
      }
      if (!this._isLastStep()) {
        this._showElem(this.nextBtn);
        this._hideElem(this.continueBtn);
      }
    };

    Instanda.multiStepForm = multiStepForm;
  })();


  /**
   * Create tooltip with help text content
   */
  (function() {
    var buildOverlay = function() {
      $('.instanda-question-item').once('overlay-popup').each(function(index, item) {
        var $icon = $('.instanda-responsive-help-icon-container a', item).removeAttr('title data-target');
        var $content = $('.instanda-responsive-help-text > div', item);

        if ($icon.length > 0 &amp;&amp; $content.length > 0) {
          var cls = $content[0].getAttribute('class').replace(/col-\S*/g, &quot;&quot;).trim().replace(/\s+/g, &quot; &quot;);
          $content[0].setAttribute('class', cls);
          tippy($icon[0], {
            content: $content[0]
          });
        }
      });
    };

    var construct = function() {
      buildOverlay();
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  $(document).ready(function () {
    //--- Show relevant cover options depending on trade
    if (Instanda.Variables.PageName == 'quickquotequestions' &amp;&amp; Instanda.Variables.PageNumber == 2) {
      switch(Instanda.Variables.TradeIdentifier_CHOICE) {
        case 'PI + Office':
          $('input#InsuranceType_CHOICE_Voslocaux, input#InsuranceType_CHOICE_Votreactivitvoslocaux').parent('label').show();
          break;
        case 'PI Only':
          $('input#InsuranceType_CHOICE_Voslocaux, input#InsuranceType_CHOICE_Votreactivitvoslocaux').parent('label').hide();
          break;
        default:
          $('input#InsuranceType_CHOICE_Voslocaux, input#InsuranceType_CHOICE_Votreactivitvoslocaux').parent('label').hide();
      }
    }

    $('header.navbar').prependTo('#innerBody');
  }); // document.ready


  /**
   * Quick Quote Questions (Page 1)
   */
  (function() {
    // do nothing if it is not quick quote questions page 1.
    if ((Instanda.Variables.PageName != 'quickquotequestions' &amp;&amp; Instanda.Variables.PageName != 'quickquotequestionscontinue')
      || Instanda.Variables.PageNumber != 1) return;

    var createMultiStepForm = function() {
      if (Instanda.utils.isMobile()) {
        $('#innerBody form').once('multiStepForm').each(function() {
          var multiStep = new Instanda.multiStepForm();
          multiStep.addForm('#question119038, #question120329');
          multiStep.addForm('#question118945');
          multiStep.setContinueBtn('[name=&quot;continueButton&quot;]');
          multiStep.setBackBtn('#backButton');
          multiStep.setNextBtn('&lt;a class=&quot;btn btn-primary&quot;>Étape suivante&lt;/a>', '[name=&quot;continueButton&quot;]', 'insertAfter');
          multiStep.setPreviousBtn('&lt;a class=&quot;btn btn-primary&quot;>Étape précédente&lt;/a>', $('&lt;div class=&quot;pull-left&quot;>&lt;/div>').insertBefore(multiStep.continueBtn.parent('.pull-right')), 'appendTo');
          multiStep.start();
        });
      }
    }

    // edit html for ease of style
    var editDom = function() {
      $('#insertActivity_MIMultiItemsHere').children('.container').removeClass('container').children('.row').removeClass('row');

      $('#insertActivity_MIMultiItemsHere').find('[class^=&quot;col-&quot;], [class*=&quot; col-&quot;]').each(function() {
        var cls = this.getAttribute('class').replace(/col-\S*/g, &quot;&quot;).trim().replace(/\s+/g, &quot; &quot;);
        this.setAttribute('class', cls);
      });

      $('#multiItemSummary_119677').children('.container').removeClass('container').children('.row').removeClass('row');

      $('#multiItemSummary_119677').find('[class^=&quot;col-&quot;], [class*=&quot; col-&quot;]').each(function() {
        var cls = this.getAttribute('class').replace(/col-\S*/g, &quot;&quot;).trim().replace(/\s+/g, &quot; &quot;);
        this.setAttribute('class', cls);
      });
    }

    // set min effective change date
    var setMinStartDate = function() {
      var $startDateInput = $('#EffectiveChangeDate');

      $startDateInput.length > 0 &amp;&amp; $startDateInput.once('setMinStartDate').each(function(elem) {
        var DatePicker = $startDateInput.parent('.instanda-question-date').data(&quot;DateTimePicker&quot;);

        if (DatePicker) {
          // bootstrap-datepicker
          DatePicker.minDate(moment().startOf('d'));
        } else {
          // native
          $startDateInput[0].min = moment().startOf('d').format('YYYY-MM-DD');
        }
      });
    }

    // Populate &quot;Je ne trouve pas mon activité&quot; when the checkbox is checked
    var populateCheckbox = function() {
      const $checkbox = $('#MissingActivity_CONF');
      const $sector = $('#Sector_CHOICE');
      const $activity = $('#Activity_CHOICE');

      const cannotFound = 'Je ne trouve pas mon activité';

      // bind autocomplete event after it built
      $('#Activity_CHOICE').closest('.instanda-question-input').on('autoComplete.built', function(event) {
        $(event.currentTarget.children[0]).on('typeahead:change typeahead:select', function() {
          if ($('.tt-input', this).val() === cannotFound) {
            $checkbox.prop('checked', true).trigger('change');
          }
        })
      });
      // make activity field readonly when checkbox checked
      $checkbox.on('change', function() {
        const $activity = $('#Activity_CHOICE');
        if ($(this).is(':checked')) {
          $activity.val(cannotFound).trigger('input').prop('readOnly', true);
        } else {
          $activity.val('').trigger('change').prop('readOnly', false);
        }
      })
      $sector.on('change', function() {
        $checkbox
          .prop('checked', false)
          .parent()
          .removeClass('ticked');
        $checkbox.closest('.questionItem').toggle($(this).val() !== '')
      })

      $checkbox.closest('.questionItem').toggle($sector.val() !== '')
    }

    var construct = function() {
      if (Instanda.Variables.PageName == 'quickquotequestions') {
        createMultiStepForm();
      }
      editDom();
      setMinStartDate();
      populateCheckbox();
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Quick Quote Questions (Page 2).
   */
  (function() {
    // do nothing if it is not quick quote questions page 2.
    if (Instanda.Variables.PageName != 'quickquotequestions' || Instanda.Variables.PageNumber != 2) return;

    var removeSelected = function() {
      $('#question118955 .instanda-question-choice label.instanda-selected').each(function(index, elem) {
        $(elem).removeClass('instanda-selected');
        var interval = setInterval(function() {
          var $input = $('input:checked', elem);
          if ($input.prop('checked')) {
            $('input:checked', elem).prop('checked', false);
          }
          else {
            clearInterval(interval);
          }
        }, 400);
      });
    }

    var triggerContinue = function() {
      $('#question118955 input[type=&quot;radio&quot;]').once('triggerContinue').on('change', function() {
        $('button[name=&quot;continueButton&quot;]').click();
      })
    }

    var _removeChoices = function() {
      //-----Map activities to cover choices on quick quote questions page 2 - PI vs Office vs both
      var option_pi = $('input#InsuranceType_CHOICE_Votreactivitprofessionnelle').closest('.instanda-question-choice');
      var option_office = $('input#InsuranceType_CHOICE_Voslocaux').closest('.instanda-question-choice');
      var option_both = $('input#InsuranceType_CHOICE_Votreactivitvoslocaux').closest('.instanda-question-choice');

      if ( Instanda.Variables.TradeIdentifier_CHOICE == 'PI Only' ) {
        option_office.remove();
        option_both.remove();
      } else if ( Instanda.Variables.TradeIdentifier_CHOICE == 'Office Only' ) {
        option_pi.remove();
        option_both.remove();
      }
    }

    var buildSwiper = function() {
      $('#question118955').once('buildSwiper').each(function(elem) {
        _removeChoices();

        var $slides = $('.instanda-question-choice', elem).addClass('swiper-slide');
        var $wrapper = $slides.parent().addClass('swiper-wrapper').wrap('&lt;div class=&quot;my-swiper-container&quot;>&lt;/div>');
        $wrapper.parent().append('&lt;div class=&quot;swiper-button-prev&quot;>&lt;/div>');
        $wrapper.parent().append('&lt;div class=&quot;swiper-button-next&quot;>&lt;/div>');

        var swiperOption = {
          watchOverflow: true,
          slidesPerView: Instanda.utils.isMobile() ? 1 : $slides.length,
          navigation: {
            nextEl: '.swiper-button-next',
            prevEl: '.swiper-button-prev',
          },
        }
        var swiper = new Swiper('.my-swiper-container', swiperOption);
      });
    }

    var construct = function() {
      removeSelected();
      triggerContinue();
      buildSwiper();
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Quick Quote Questions (Page 3)
   */
  (function() {
    // do nothing if it is not quick quote questions page 3.
    if ((Instanda.Variables.PageName != 'quickquotequestions' &amp;&amp; Instanda.Variables.PageName != 'quickquotequestionscontinue')
      || Instanda.Variables.PageNumber != 3) return;

    var createMultiStepForm = function() {
      if (Instanda.utils.isMobile()) {
        $('#innerBody form').once('multiStepForm').each(function() {
          var multiStep = new Instanda.multiStepForm();
          multiStep.addForm('.instanda-quick-quote-questions > .instanda-questionList > .instanda-well:nth-child(2)');
          multiStep.addForm('.instanda-quick-quote-questions > .instanda-questionList > .instanda-well:nth-child(3)');
          multiStep.setContinueBtn('[name=&quot;continueButton&quot;]');
          multiStep.setBackBtn('#backButton');
          multiStep.setNextBtn('&lt;a class=&quot;btn btn-primary&quot;>Étape suivante&lt;/a>', '[name=&quot;continueButton&quot;]', 'insertAfter');
          multiStep.setPreviousBtn('&lt;a class=&quot;btn btn-primary&quot;>Étape précédente&lt;/a>', $(multiStep.continueBtn.parent('.pull-right')).siblings('.pull-left'), 'appendTo');
          multiStep.start();
        });
      }
    }

    var addAutoComplete = function() {
      $('#EnterpriseName_TXT').attr('autocomplete', 'organization');
      $('#AddressLine1').attr('autocomplete', 'address-line1');
      $('#AddressLine2').attr('autocomplete', 'address-line2');
      $('#City').attr('autocomplete', 'address-level2');
      $('#Postcode118934').attr('autocomplete', 'postal-code');
    }

    var construct = function() {
      if (Instanda.Variables.PageName == 'quickquotequestions') {
        createMultiStepForm();
      }
      addAutoComplete();
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Quick Quote Questions (Page 4)
   */
  (function() {
    if ((Instanda.Variables.PageName == 'quickquotequestions' || Instanda.Variables.PageName == 'quickquotequestionscontinue')
    &amp;&amp; Instanda.Variables.PageNumber == 4) {

      // Fix the missing leading 0 on postcode field
      const fixPostCode = function() {
        const field = $('#Postcode_FormB_NUM');
        let value = field.val().replace(/\s+/,'')
        if (value.match(/^\d+$/)) {
          value = value.padStart(5, 0);
        }
        field.val(value).trigger('change');
      }

      // Auto fill activity choice to Form B
      const autoFillActivityChoice = function() {
        setTimeout(function() {
          $('#Activity_CHOICE_FormB_TXT').closest('.instanda-question-input').on('autoComplete.built', function(event) {
            $(event.currentTarget.children[0]).find('.tt-input').typeahead('val', Instanda.Variables.Activity_CHOICE);
          });
          $('#Sector_CHOICE_FormB_TXT').val(Instanda.Variables.Sector_CHOICE).trigger('change');
        }, 1000)
      }

      const populateTurnoverValue = function() {
        setTimeout(function() {
          $('#Turnover_FormB_Num').val(Instanda.Variables.Turnover_NUM).trigger('change');
        }, 1000)
      }

      $(window).on('load', autoFillActivityChoice);
      $(window).on('load', populateTurnoverValue);
      $(window).on('load', fixPostCode);

      // $(document).ready(function() {
      //   autoFillActivityChoice();
      //   populateTurnoverValue();
      //   fixPostCode();
      // });
    }
  })();


  /**
   * Quick Quote (Page 1).
   */
  (function() {
    // do nothing if it is not quick quote page 1.
    if (Instanda.Variables.PageName != 'quickquote' || Instanda.Variables.PageNumber != 1) return;

    // Move radio button to each plan
    var moveRadioBtn = function() {
      $('#PackSelect_Public_txt_Essential').appendTo('.cover-01 .cover-content');
      $('#PackSelect_Public_txt_Equilibre').appendTo('.cover-02 .cover-content');
      $('#PackSelect_Public_txt_Integral').appendTo('.cover-03 .cover-content');
    }

    // disable continue button if no radio checked
    var disableContinueBtn = function() {
      var $radios = $('.cover input[type=&quot;radio&quot;]');
      var $btn = $('#continueButton');
      var hasChecked = false;
      if ($btn.length > 0) {
        $radios.each(function(index, radio) {
          hasChecked = hasChecked || radio.checked;
        });
        $btn[0].disabled = !hasChecked;
      }
    }

// sync items across covers in 3 boxes
    var syncCoverItems = function() {
      var $list = $('.cover-list').once('syncCoverItems');
      if ($list.length > 0 &amp;&amp; !(Instanda.Variables.Sector_CHOICE == 'Informatique' &amp;&amp; Instanda.Variables.Turnover_NUM &lt;= 95000 &amp;&amp; Instanda.Variables.InsuranceType_CHOICE == 'Votre activité professionnelle')){
        var $covers = $('.cover-wrapper', $list);
        var lastIndex = $covers.length - 1;

	//Commented by (DanielPeresFR ) - APFD-1233 and APFD-1234

       /* var removeLastCover =
          (Instanda.Variables.Sector_CHOICE === 'Véhicule de Tourisme avec Chauffeur') ||
          (Instanda.Variables.Sector_CHOICE === 'Artisans, Commerçants et e-Commerçants de détail'
            &amp;&amp; Instanda.Variables.InsuranceType_CHOICE === 'Votre activité professionnelle');

        if (removeLastCover) {
          $covers.last().remove();
          var lastIndex = $covers.length - 2;
        }*/

        var $lastCover = $covers[lastIndex];
        $('.cover-content > div', $lastCover).each(function(itemIndex, item) {
          $covers.slice(0, lastIndex).each(function(coverIndex, cover) {
            const $coverItems = $('.cover-content > div', cover)
            const length = $coverItems.length;
            if (itemIndex >= length) {
              $('.cover-content input', cover).before($(item).clone().addClass('grey-out'));
            }
          });
        });
      }
    }

    var toggleSelectedClass = function() {
      const toggleClass = function(radio, box) {
        var checked = $(radio).prop('checked');
        if (checked) {
          $(box).addClass('instanda-selected')
          $(boxes).not(box).removeClass('instanda-selected');
        } else {
          $(box).removeClass('instanda-selected');
        }
      }
      const boxes = $('.cover-wrapper > .box');

      boxes
        .each(function(index, box) {
          var $radio = $('input[type=&quot;radio&quot;]', box)
          toggleClass($radio, box, boxes);
        })
        .once('coverWrapperClick')
        .on('click', function(event) {
          const box = event.currentTarget;
          event.stopPropagation();
          var $radio = $('input[type=&quot;radio&quot;]', box);
          $radio.prop('checked', true).trigger('change');
          toggleClass($radio, box, boxes);
        });
    }

    var buildSwiper = function() {
      $('#instanda-quote-content').once('buildSwiper').each(function(elem) {
        var $slides = $('.cover-list > .cover-wrapper', elem).addClass('swiper-slide');
        var $wrapper = $slides.parent().addClass('swiper-wrapper').wrap('&lt;div class=&quot;my-swiper-container&quot;>&lt;/div>');
        $wrapper.parent().append('&lt;div class=&quot;swiper-button-prev&quot;>&lt;/div>');
        $wrapper.parent().append('&lt;div class=&quot;swiper-button-next&quot;>&lt;/div>');

        var swiperOption = {
          watchOverflow: true,
          slidesPerView: Instanda.utils.isMobile() ? 1 : $slides.length,
          navigation: {
            nextEl: '.swiper-button-next',
            prevEl: '.swiper-button-prev',
          }
        }
        var swiper = new Swiper('.my-swiper-container', swiperOption);
        swiper.slideTo(1);
      });
    }

    var buildToolTips = function() {
      $('.help-text').once('overlay-popup').each(function(index, item) {
        var $icon = $('a', item);
        var $content = $('&lt;div>' + $icon.data('content') + '&lt;/div>');

        if ($icon.length > 0 &amp;&amp; $content.length > 0) {
          tippy($icon[0], {
            content: $content[0]
          });
        }
      });
    };

    var construct = function() {
      moveRadioBtn();
      syncCoverItems();
      toggleSelectedClass();
      buildSwiper();
      buildToolTips();
      disableContinueBtn();
    }

    $(window).on('load', function() {
      setTimeout(function() {
        toggleSelectedClass();
      }, 0);
    });
    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Pre Quote Questions (Page 1).
   */
  (function() {
    // do nothing if it is not pre quote questions page 1.
    if ((Instanda.Variables.PageName != 'prequotequestions'
      &amp;&amp; Instanda.Variables.PageName != 'prequotequestionscontinue')
      || Instanda.Variables.PageNumber != 1) return;

    var createMultiStepForm = function() {
      if (Instanda.utils.isMobile()) {
        $('#innerBody form').once('multiStepForm').each(function() {
          var multiStep = new Instanda.multiStepForm();
          multiStep.addForm('.instanda-pre-quote-questions > .instanda-questionList > .instanda-well:nth-child(2)');
          multiStep.addForm('.instanda-pre-quote-questions > .instanda-questionList > .instanda-well:nth-child(3)');
          multiStep.setContinueBtn('[name=&quot;continueButton&quot;]');
          multiStep.setBackBtn('#backButton');
          multiStep.setNextBtn('&lt;a class=&quot;btn btn-primary&quot;>Étape suivante&lt;/a>', '[name=&quot;continueButton&quot;]', 'insertAfter');
          multiStep.setPreviousBtn('&lt;a class=&quot;btn btn-primary&quot;>Étape précédente&lt;/a>', $(multiStep.continueBtn.parent('.pull-right')).siblings('.pull-left'), 'appendTo');
          multiStep.start();
        });
      }
    }

    // pre-fill address
    var autofillAddress = function() {
      setTimeout(function() {
        if (Instanda.Variables.RiskAddress_YN === &quot;Confirmed&quot;) {
          $('#CompanyName_TXT').val(Instanda.Variables.EnterpriseName_TXT);
          $('#CompanyName_TXT--mdc').val(Instanda.Variables.EnterpriseName_TXT).trigger('change');
          $('#CompanyAddressLine_TXT').val([Instanda.Variables.AddressLine1,Instanda.Variables.AddressLine2].join(' ').trim());
          $('#CompanyAddressLine_TXT--mdc').val([Instanda.Variables.AddressLine1,Instanda.Variables.AddressLine2].join(' ').trim()).trigger('change');
          $('#CompanyPostcode_TXT').val(Instanda.Variables.Postcode.toString().padStart(5, '0'));
          $('#CompanyPostcode_TXT--mdc').val(Instanda.Variables.Postcode.toString().padStart(5, '0')).trigger('change');
          $('#CompanyCity_TXT').val(Instanda.Variables.City);
          $('#CompanyCity_TXT--mdc').val(Instanda.Variables.City).trigger('change');
        }
      }, 1000);
    }

    // set autocomplete attributes
    var addAutoComplete = function() {
      $('#FirstName').attr('autocomplete', 'given-name');
      $('#LastName').attr('autocomplete', 'family-name');
      $('#Email_TXT').attr('autocomplete', 'email');
      $('#TelephoneNumber_NUM').attr('autocomplete', 'tel');
      $('#CompanyName_TXT').attr('autocomplete', 'organization');
      $('#CompanyAddressLine_TXT').attr('autocomplete', 'street-address');
      $('#CompanyPostcode_TXT').attr('autocomplete', 'postal-code');
    }

    var construct = function() {
      if (Instanda.Variables.PageName == 'prequotequestions') {
        createMultiStepForm();
      }
      addAutoComplete();
    }

    $(window).on('load', autofillAddress);

    $(document)
      .ready(function() {
        construct();
      })
      .ajaxComplete(construct);
  })();


  /**
   * Pre Quote Questions (Page 2).
   */
  (function() {
    // do nothing if it is not pre quote questions page 2.
    if (Instanda.Variables.PageName != 'prequotequestions' || Instanda.Variables.PageNumber != 2) return;

    // remove bootstrap grid class for ease of style
    var removeColClass = function() {
      $('.instanda-main-content .questionList').find('[class^=&quot;col-&quot;], [class*=&quot; col-&quot;]').each(function() {
        var cls = this.getAttribute('class').replace(/col-\S*/g, &quot;&quot;).trim().replace(/\s+/g, &quot; &quot;);
        this.setAttribute('class', cls);
      });
    }

    // move tool tip icon
    var moveIcon = function() {
      $('.instanda-main-content .questionList').find('.instanda-question-help-col').once('moveIcon').each(function() {
        var $iconContainer = $(this);
        $iconContainer.parent().parent().append($iconContainer);
      });
    }

    var construct = function() {
      removeColClass();
      moveIcon();
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Pre Quote Questions (Page 3).
   */
  (function() {
    // do nothing if it is not pre quote questions page 3.
    if (Instanda.Variables.PageName != 'prequotequestions' || Instanda.Variables.PageNumber != 3) return;

    // set min policy start date
    var setMinStartDate = function() {
      var $continueBtn = $('[name=&quot;continueButton&quot;]');
      var $startDateInput = $('#PolicyStartDate_DATE');

      $startDateInput.once('setMinStartDate').each(function(elem) {
        var DatePicker = $startDateInput.parent('.instanda-question-date').data(&quot;DateTimePicker&quot;);

        if (DatePicker) {
          // bootstrap-datepicker
          DatePicker.minDate(moment().startOf('d'));
        } else {
          // native
          $startDateInput[0].min = moment().startOf('d').format('YYYY-MM-DD');
        }
        $startDateInput.on('focus', function() {
          $continueBtn.prop('disabled', true);
        }).on('blur', function() {
          $continueBtn.prop('disabled', false);
        });
      });
    }

    var construct = function() {
      setMinStartDate()
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Move &quot;instanda-well&quot; help text banner
   */
  (function() {
    var moved = false;

    var _moveBanner = function(target) {
      const $banner = $('.instanda-main-content').find(target);
      if (!moved &amp;&amp; $banner.length > 0) {
        $banner.prependTo('.instanda-main-content-container');
        moved = true;
      } else {
        $banner.remove();
      }
    }

    var moveBanner = function() {
      if (Instanda.Variables.PageName == 'prequotequestions' &amp;&amp; Instanda.Variables.PageNumber == 1) {
        _moveBanner('#question118956');
      }
      if (Instanda.Variables.PageName == 'prequotequestions' &amp;&amp; Instanda.Variables.PageNumber == 2) {
        _moveBanner('#question120226');
      }
      if (Instanda.Variables.PageName == 'prequotequestions' &amp;&amp; Instanda.Variables.PageNumber == 3) {
        _moveBanner('#question120292');
      }
      if (Instanda.Variables.PageName == 'quickquote' &amp;&amp; Instanda.Variables.PageNumber == 1) {
        _moveBanner('#instanda-quote-content .questionList');
      }
      if (Instanda.Variables.PageName == 'postquotequestions') {
        _moveBanner('#question120291');
        _moveBanner('#question119073');
        _moveBanner('#question120358');
      }
    }

    $(document)
      .ready(moveBanner)
      .ajaxComplete(moveBanner);
  })();


  /////////////////////////////////////////////////////////////////////////////// CHECKBOXES
  (function() {
    //------Add class to selected options on questions and covers on all pages
    function add_class_to_selected_option_on_question_and_quote_pages_for_checkboxes() {
      $('label input[type=&quot;checkbox&quot;]').each(function(){
        if ($(this).is(':checked')) {
          $(this).parent('label').addClass('ticked').removeClass('unticked');
        } else {
          $(this).parent('label').addClass('unticked').removeClass('ticked');
        }
      });
    }

    //-----When clicked, on question pages
    $('body').on('click change', 'label input[type=&quot;checkbox&quot;]', function(){
      if ($(this).is(':checked')) {
        $(this).parent('label').addClass('ticked').removeClass('unticked');
      } else {
        $(this).parent('label').addClass('unticked').removeClass('ticked');
      }
    } );

    //-----Add class to selected options on questions and covers on all pages
    function add_class_to_disabled_option_on_question_and_quote_pages_for_checkboxes() {
      $('label input[type=&quot;checkbox&quot;][disabled=&quot;disabled&quot;]').each(function(){
        $(this).parent('label').addClass('disabled');
      });
    }

    //-----Run----------
    $(document).ready(function () {
      add_class_to_selected_option_on_question_and_quote_pages_for_checkboxes();
      add_class_to_disabled_option_on_question_and_quote_pages_for_checkboxes();
    });

    $(document).ajaxSuccess(function() {
      add_class_to_selected_option_on_question_and_quote_pages_for_checkboxes();
      add_class_to_disabled_option_on_question_and_quote_pages_for_checkboxes();
    });
  })();


  /**
   * Customer Portal.
   */
  (function() {
    setTimeout(function() {
      // open tab if there is hash value on url
      if (document.location.hash.length > 0) {
        $('.nav-item a[href=&quot;'+document.location.hash+'&quot;]').tab('show');
      }

      // change navigate history when user switch tabs.
      $('.nav-item a').on('shown.bs.tab', function(e) {
        if (history.pushState) {
          window.history.pushState(null, null, e.target.hash);
        }
      });
    }, 0);

    if (Instanda.Variables.PageName != 'customerhomepage') return;

    $('.accordion-doc [data-toggle=&quot;collapse&quot;]').on('click', function(event) {
      var collapse = $(event.target).next().collapse('toggle');
      $(event.target).attr('aria-expanded', collapse.attr('aria-expanded'));
    });
  })();


  /**
   * Customer Portal Edit Details Page.
   *
   * Insert content to Edit Customer Details page
   */
  (function() {
    if (Instanda.Variables.PageName === 'customerdetailschangepassword'){
      $('.page-customerdetailschangepassword .instanda-main-content').prepend('&lt;div class=&quot;container&quot;>&lt;h3>Modification de votre mot de passe&lt;/h3>&lt;p style=&quot;margin-bottom: 40px;&quot;>Merci de renseigner ci-dessous votre nouveau mot de passe.&lt;/p>&lt;/div>');
    }
  })();


  /**
   * Customer Portal Password Reset Sent.
   *
   * Add &quot;alert-success&quot; class to password reset succeeded message
   */
  (function() {
    if (Instanda.Variables.PageName === 'customersendpasswordresetlink'){
      $('#instanda-cp-reset-link-form .section_content').addClass('alert alert-success');
    }
  })();


  /////////////////////////////////////////////////////////////////////////////// Translation
  (function() {
    // Post quote questions page 1.
    if (Instanda.Variables.PageName === 'postquotequestions' &amp;&amp; Instanda.Variables.PageNumber === 1) {
      $('button[name=&quot;continueButton&quot;]').html('Souscrire en ligne');
    }

    // Customer Portal Login Page
    if (Instanda.Variables.PageName === 'customerlogin') {
      $('button[type=&quot;submit&quot;]').text('Me connecter');
      $('#Password').attr('placeholder','Votre mot de passe');
      $('input[placeholder=&quot;email address&quot;]').attr('placeholder','Votre email');
      $('#instanda-cp-forgot-link').text('Je ne retrouve plus mon mot de passe');
    }

    // Customer Portal Forget Password and Send Password Reset Pages
    if (Instanda.Variables.PageName === 'customersendpasswordresetlink') {
      $('#instanda-cp-user-name').attr('placeholder','Identifiant (email)');
      $('button[type=&quot;submit&quot;]').text('Recevoir mon mot de passe par email');
      $('#instanda-cp-reset-link-form a').html('Renvoyer').attr('data-loading-text','Renvoyer');
    }

    // Customer Portal Input and Confirm New Password Page
    if (Instanda.Variables.PageName === 'customerforgotpassword') {
      $('#NewPassword').attr('placeholder','Votre nouveau mot de passe');
      $('#ConfirmPassword').attr('placeholder','Confirmez votre nouveau mot de passe');
      $('#instanda-cp-charge-password').text('Changer mon mot de passe').attr('data-loading-text','Changer mon mot de passe');
    }

    // Customer Portal Your Account Details Page
    if (Instanda.Variables.PageName === 'customerdetails') {
      $('label.control-label:contains(&quot;Username&quot;)').text('Identifiant (email)');
      $('label.control-label:contains(&quot;Password&quot;)').text('Mot de passe');
      $('#password-fake-label + a').html('Modifier');
      $('.instanda-update-customer-details-button').text('Mettre à jour');
      $('#savemsg strong').html('Vos coordonnées ont été mises à jour');
    }

    // Customer Portal Change Password Page (enter from Your Account Details Page)
    if (Instanda.Variables.PageName === 'customerdetailschangepassword') {
      $('label.control-label:contains(&quot;Old&quot;)').text('Votre mot de passe');
      $('label.control-label:contains(&quot;Confirm&quot;)').text('Confirmez votre nouveau mot de passe');
      $('label.control-label:contains(&quot;New&quot;)').text('Votre nouveau mot de passe');
      $('#instanda-save-password-button').text('Mettre à jour').attr('data-loading-text','enregistrer');
      $('form a:contains(&quot;Cancel&quot;)').html('Annuler');
    }

    // Registration
    if (Instanda.Variables.PageName === 'existingpolicyholdersendregistrationlink' || Instanda.Variables.PageName === 'publicuatuserlogin' || Instanda.Variables.PageName === 'customerregister') {
      $('#instanda-cp-user-name').attr('placeholder','Identifiant (email)');
      $('button[type=&quot;submit&quot;]').text('Continuer').attr('data-loading-text','Continuer');
      $('label.control-label:contains(&quot;E-mail address&quot;)').text('Votre email');
      $('label.control-label:contains(&quot;Password&quot;)').text('Votre mot de passe');
    }

    // Registration error message contains dynamic data, email address
    $('.validation-summary-errors li').each(function(index, elem) {
      const text = $(elem).text();
      const regex = /E-mail address '(.*)' has already been registered./;
      const matches = text.match(regex);

      if (matches) {
        $(elem).text('Un compte existe déjà pour cette adresse e-mail. Merci de cliquer sur \'Me connecter\' pour accéder à votre Espace client.');
      }
    });

    const translations = {
      'Log Out': 'Déconnexion',
      'Log in': 'Me connecter',
      'Your Account': 'Vos coordonnées',
      'Continue': 'Étape suivante',
      'Email address is required': 'Adresse e-mail est nécessaire',
      'Please use a valid email address.': 'Veuillez utiliser une adresse mail valide',
      'The Password field is required.': 'Le champ Mot de passe est obligatoire',
      'Password is required': 'Mot de passe requis',
      'New password is required': 'Un nouveau mot de passe est requis',
      'Confirm password is required': 'Confirmer le mot de passe est requis',
      'City': 'Ville',
      'Password must be at least 8 characters.': 'Le mot de passe doit contenir au moins 8 caractères',
      'Password must contain at least 1 numbers.': 'Le mot de passe doit contenir au moins 1 chiffre',
      'Passwords must include at least 1 special characters (!#@ etc)': 'Le mot de passe doit contenir au moins 1 caractère spécial (!#@ etc)',
      'Invalid password': 'Mot de passe incorrect',
      'Passwords do not match': 'Les mots de passe ne correspondent pas',
      'Password must be between 6 and 99 characters': 'Le mot de passe doit comporter entre 6 et 99 caractères',
      'User is already registered': 'Un compte lié à cette adresse email existe déjà',
      'If that email address is in our database, we will send you an email to reset your password.': 'Les instructions pour changer / recouvrer votre mot de passe vous ont été envoyés par email.',
      'Policy start date cannot be in the past': 'La date d\'effet du contrat ne peut pas être dans le passé. Pour finaliser votre contrat, merci de choisir une nouvelle date d\'effet. (+ indiquer qu\'il faut cliquer sur \'Précédent\'?',
      'Authentication failed.': 'Échec de l\'authentification',
      'You are logged in with a different email address, you need to logout first to proceed with this quote': 'Vous êtes connecté avec une adresse e-mail différente, vous devez d\’abord vous déconnecter pour continuer avec ce devis',
'Logout and return to quote': 'Déconnecter pour continuer avec ce devis',
    }

    // @see https://stackoverflow.com/a/42041097/1146907
    const innerBody = document.getElementById('innerBody');
    const walker = document.createTreeWalker(innerBody, NodeFilter.SHOW_TEXT, null, false);
    while (walker.nextNode()) {
      for (const source in translations) {
        if (walker.currentNode.nodeValue.trim() == source) {
          walker.currentNode.nodeValue = translations[source];
        }
      }
    }

    const messageFields = document.querySelectorAll('[data-valmsg-for]');
    const callback = function(mutationList, observer) {
      for (let mutation of mutationList) {
        for (const source in translations) {
          if (mutation.addedNodes &amp;&amp; mutation.addedNodes.length > 0 &amp;&amp; mutation.addedNodes[0].innerText.trim() == source) {
            mutation.addedNodes[0].innerText = translations[source];
          }
        }
      }
    }
    const observer = new MutationObserver(callback);
    const config = { childList: true };
    messageFields.forEach(function(field) {
      observer.observe(field, config);
    });

  })();


// HXF-3 - Postcode look up
const lookUpPostcode = function() {
  var _PostcodeFields = [];
  _PostcodeFields.push({
    postcode: &quot;input#Postcode118934&quot;,
    city: &quot;input#City&quot;,
    container: &quot;.form-group&quot;,
    wrapper: '&lt;div class=&quot;instanda-question-item form-group container postcode-look-up&quot;>&lt;/div>'
  }); // Page 3
  _PostcodeFields.push({
    postcode: &quot;input#Postcode_FormB_NUM&quot;,
    city: &quot;input#City_FormB&quot;,
    container: &quot;.questionItem&quot;,
    wrapper: '&lt;div class=&quot;questionItem instanda-question-item form-group container postcode-look-up&quot;>&lt;/div>'
  }); // Page 4
  _PostcodeFields.push({
    postcode: &quot;input#CompanyPostcode_TXT&quot;,
    city: &quot;input#CompanyCity_TXT&quot;,
    container: &quot;.questionItem&quot;,
    wrapper: '&lt;div class=&quot;questionItem instanda-question-item form-group container postcode-look-up&quot;>&lt;/div>'
  }); // Page 5

  $.each(_PostcodeFields, function(key, value){
    $(value.city).prop('readonly', true).trigger(&quot;mdc:rebuild&quot;);

    window.setTimeout(function () {
      if ($(value.postcode).length > 0) {
        $(value.postcode).on(&quot;change&quot;, function () {
          if (typeof $(value.postcode).val() !== &quot;undefined&quot; &amp;&amp; $(value.postcode).val().trim() !== &quot;&quot;) {
            $.get(&quot;https://geo.api.gouv.fr/communes&quot;, { codePostal: $(value.postcode).val().replace(/ /g, '') }).done(
              function (data) {
                if ($(&quot;select.gc-fr-postale-code&quot;).length > 0) {
                  $(&quot;select.gc-fr-postale-code&quot;).closest('.instanda-question-item').remove();
                }
                $(value.city+'--mdc').focus();
                $(value.city).val(&quot;&quot;).trigger(&quot;change&quot;);
                if (data.length > 0) {
                  var _postal = data;
                  if (typeof _postal !== &quot;undefined&quot; &amp;&amp; _postal !== null) {
                    if(_postal.length === 1){
                      $(value.city).val(_postal[0].nom).trigger(&quot;change&quot;);
                    } else {
                      var _ddl = $(&quot;&lt;select>&quot;)
                        .addClass(&quot;gc-fr-postale-code&quot;)
                        .on(&quot;change&quot;, function(){
                          $(value.city).val($(this).val()).trigger(&quot;mdc:rebuild&quot;);
                        });
                      $.each(_postal, function(key, value){
                        $(_ddl).append(
                          $(&quot;&lt;option/>&quot;)
                          .attr({ value: value.nom })
                          .text(value.nom)
                        );
                      });
                      $(value.city).val(_ddl.val()).trigger(&quot;mdc:rebuild&quot;);
                      var _ddlContainer = $(value.wrapper).append(_ddl);
                      $(value.city+'--mdc').closest(value.container).before(_ddlContainer);
                    }
                  }
                } else {
                  //$(value.city).focus();
                  $(value.city).val(&quot;&quot;); //.trigger(&quot;change&quot;);
                }
              }
            );
          }
        });
      }
    });
  },2000);
}

$(document).ready(lookUpPostcode);



  /* Quote Display */
  (function () {
    // send email and continue in one click
    if (Instanda.Variables.PageName == 'quote' &amp;&amp; (Instanda.Variables.PageNumber == 1 || Instanda.Variables.PageNumber == 5 )) {
      const fakeBtn = $('.sidebar-cta .btn-fake');
      const nextBtn = $('.sidebar-cta .instanda_nav_link').hide();
      const nextBtnBelow = $('#continueButton').hide();
      const fakeBtnBelow = $('&lt;button type=&quot;submit&quot; name=&quot;continueButton&quot; class=&quot;instanda-button instanda-quote-continue-button btn btn-primary&quot;>Recevoir mon devis par email &lt;i class=&quot;fa fa-caret-right&quot;>&lt;/i>&lt;/button>').insertBefore(nextBtnBelow); 
      const emailLink = $('#emailQuoteLinkLink').hide();

      fakeBtnBelow.on('click', function () {
        emailLink.trigger('click');
        /* commented by RitaHiscoxFrance 13/04/2022 INST #20373 - begin {
        setTimeout(function () {
          nextBtnBelow.trigger('click');
        }, 200);
        \* end } - commented by RitaHiscoxFrance 13/04/2022 INST #20373 */
      });

      fakeBtn.on('click', function () {
        emailLink.trigger('click');
        setTimeout(function () {
          nextBtn.trigger('click');
        }, 200);
      });
    }
  })();



  // swap address fields order
  (function() {
    const addressContainer = $('#question118934');
    if (addressContainer.length > 0) {
      const postCode = $('.instanda-address-postcode', addressContainer).closest('.instanda-question-item');
      const city = $('.instanda-address-city', addressContainer).closest('.form-group');
      if (postCode.length > 0 &amp;&amp; city.length > 0) {
        postCode.insertBefore(city);
      }
    }
  })();


  /**
   * Post-Quote Questions (Page 1).
   */
  (function() {
    // set download document URL
    if (Instanda.Variables.PageName === 'postquotequestions' &amp;&amp; Instanda.Variables.PageNumber == '1') {
      const matches = document.location.search.match(/[?|&amp;]quoteRef=(.*?)[$|&amp;]/);
      const quoteRef = matches[1];

      if (Instanda.Variables.InsuranceType_CHOICE === 'Votre activité professionnelle') {
        const docLink = '/Public/DownloadStoredPdfBuilder?packageId=' + '12462' + '&amp;pdfBuilderId=' + '305624' + '&amp;quoteRef=' + quoteRef;
        const link = $('.download-link').attr('href', docLink);
      }

      if (Instanda.Variables.InsuranceType_CHOICE === 'Vos locaux') {
        const docLink = '/Public/DownloadStoredPdfBuilder?packageId=' + '12462' + '&amp;pdfBuilderId=' + '305637' + '&amp;quoteRef=' + quoteRef;
        const link = $('.download-link').attr('href', docLink);
      }

      if (Instanda.Variables.InsuranceType_CHOICE === 'Votre activité &amp; vos locaux') {
        const docLink = '/Public/DownloadStoredPdfBuilder?packageId=' + '12462' + '&amp;pdfBuilderId=' + '305652' + '&amp;quoteRef=' + quoteRef;
        const link = $('.download-link').attr('href', docLink);
      }
    }
  })();


  /**
   * Post-Quote Questions (Page 2).
   */
  (function() {
    // set download document URL
    if ((Instanda.Variables.PageName === 'postquotequestions' &amp;&amp; Instanda.Variables.PageNumber == '2') || (Instanda.Variables.PageName === 'postquotequestionscontinue' &amp;&amp; Instanda.Variables.PageNumber == '2')) {
      const modifyLink = function() {
        const $modifyLink = $('.policy-start-date [data-placeholder=&quot;~publicPreQuoteLink&quot;]');
        const href = $modifyLink.prop('href');
        const url = new URL(href);
        url.searchParams.set('pageNumber', 3);
        url.searchParams.set('goBackwards', 'True');
        $modifyLink.prop('href', url.toString());
      }
      $(document).ready(modifyLink);

      const fillDummyAddress = function() {
        setTimeout(function () {
            $('#InsuredAddressLine1_TXT').val('1').trigger('change');
            $('#InsuredAddressLine2_TXT').val('Street').trigger('change');
            $('#InsuredAddressCity_TXT').val('City').trigger('change');
            $('input[name=&quot;InsuredPostcode_TXT&quot;]').val('12345').trigger('change');
        }, 0);
      }
      $(window).on('load', fillDummyAddress);
    }
  })();


  /* Phone widget */
  (function() {
    $('.tel-icon-wannaspeak').click(function() {
      wsCall();
    });
  })();


// START
// HXF-8: [PI Public] Real time class toggling on valid fields
// GC 01/10/2020
$(document).ready(function(){
    // Creates and returns a unique id
    function uuidv4() {
      return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
        var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r &amp; 0x3 | 0x8);
        return v.toString(16);
      });
    };
    // Configure our fields
    var _fieldsToValidate = [];
    _fieldsToValidate.push({ &quot;ValidOn&quot;: &quot;range&quot;, &quot;Parent&quot;: &quot;div#question119075&quot;, &quot;Selector&quot;: &quot;input#IBAN_TXT&quot;, &quot;Type&quot;: &quot;Text&quot;, &quot;Mdc&quot;: true, &quot;On&quot;: &quot;change input&quot;, &quot;Min&quot;: 27, &quot;Max&quot;: 27, &quot;Css&quot;: &quot;realtime-is-valid&quot; });
    _fieldsToValidate.push({ &quot;ValidOn&quot;: &quot;other&quot;, &quot;Parent&quot;: &quot;div#question119076&quot;, &quot;Selector&quot;: &quot;input#BIC_TXT&quot;, &quot;Type&quot;: &quot;Text&quot;, &quot;Mdc&quot;: true, &quot;On&quot;: &quot;change input&quot;, &quot;Min&quot;: 8, &quot;Max&quot;: 11, &quot;Css&quot;: &quot;realtime-is-valid&quot; });
    _fieldsToValidate.push({ &quot;ValidOn&quot;: &quot;range&quot;, &quot;Parent&quot;: &quot;div#question119077&quot;, &quot;Selector&quot;: &quot;input#Domiciliation_TXT&quot;, &quot;Type&quot;: &quot;Text&quot;, &quot;Mdc&quot;: true, &quot;On&quot;: &quot;change input&quot;, &quot;Min&quot;: 1, &quot;Max&quot;: 999, &quot;Css&quot;: &quot;realtime-is-valid&quot; });
    _fieldsToValidate.push({ &quot;ValidOn&quot;: &quot;range&quot;, &quot;Parent&quot;: &quot;div#question119078&quot;, &quot;Selector&quot;: &quot;input#AccountOwner_TXT&quot;, &quot;Type&quot;: &quot;Text&quot;, &quot;Mdc&quot;: true, &quot;On&quot;: &quot;change input&quot;, &quot;Min&quot;: 1, &quot;Max&quot;: 999, &quot;Css&quot;: &quot;realtime-is-valid&quot; });
    // Initialize our fields
    // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
    // GC 08/10/2020
    // Switch statement added to increase the scope of the validation.
    // BIC is now not a range between 8 and 11 but is either 8 OR 11.
    $.each(_fieldsToValidate, function(key, value){
        if($(value.Parent).length > 0){
            var _guid = uuidv4();
            value.Guid = _guid; // for use when validating
            $(&quot;&lt;span data-span-guid='&quot; + _guid + &quot;' class='realtime-valid' />&quot;).prependTo(value.Parent);
            // Set tab-index for focus capture
            $(value.Parent).attr({ &quot;data-realtime&quot;: JSON.stringify(value) }).attr({ &quot;tab-index&quot;: key });
            $(value.Parent).on(&quot;click&quot;, function(){
                // When we are focused in the question div bind the behaviour...
                // ...mdc compontent strips it at every focus out...
                var _data = $(this).attr(&quot;data-realtime&quot;); // data attached to question div...
                _data = JSON.parse(_data);
                $(_data.Selector, _data.Parent).data(_data).on(_data.On, function(el){
                    var _d = $(el.target).data(); // data attached to input...
                    var _thisVal = $(_d.Selector, _d.Parent).val();
                    // Check the value matches the input and hide/show css class
                    console.log(&quot;_d ================> &quot;, _d);
                    if(typeof _d !== &quot;undefined&quot; &amp;&amp; typeof _d.ValidOn !== &quot;undefined&quot;){
                        switch(_d.ValidOn.toLowerCase()){
                            case &quot;range&quot;:
                                if (typeof _thisVal !== &quot;undefined&quot; &amp;&amp; _thisVal !== null &amp;&amp; _thisVal.length >= _d.Min &amp;&amp; _thisVal.length &lt;= _d.Max) {
                                    $(_d.Parent).addClass(&quot;realtime-is-valid&quot;);
                                    $(&quot;span[data-span-guid='&quot; + _d.Guid + &quot;']&quot;).addClass(_d.Css);
                                } else {
                                    $(_d.Parent).removeClass(&quot;realtime-is-valid&quot;);
                                    $(&quot;span[data-span-guid='&quot; + _d.Guid + &quot;']&quot;).removeClass(_d.Css);
                                }
                                break;
                            case &quot;other&quot;:
                                if (typeof _thisVal !== &quot;undefined&quot; &amp;&amp; _thisVal !== null &amp;&amp; (_thisVal.length === _d.Min || _thisVal.length === _d.Max)) {
                                    $(_d.Parent).addClass(&quot;realtime-is-valid&quot;);
                                    $(&quot;span[data-span-guid='&quot; + _d.Guid + &quot;']&quot;).addClass(_d.Css);
                                } else {
                                    $(_d.Parent).removeClass(&quot;realtime-is-valid&quot;);
                                    $(&quot;span[data-span-guid='&quot; + _d.Guid + &quot;']&quot;).removeClass(_d.Css);
                                }
                                break;
                        }
                    }
                });
            });
            // GC added to trigger the code to check for previous values and autocompletes...
            setTimeout(function(){
                $(value.Parent).trigger(&quot;click&quot;);
                $(value.Selector + (value.Mdc ? &quot;--mdc&quot; : &quot;&quot;), value.Parent).keyup();
            }, 500);
        }
    });
    // GC 08/10/2020
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
});
// END
// HXF-8: [PI Public] Real time class toggling on valid fields
// GC 01/10/2020


  //////////////////////////////////////////////////////////////////////////////// LOADING ANIMATION

  // Wait for window load, custom-made loading effect
  $(document).ready(function() {
    // Animate loader off screen
    $(&quot;.custom-loading-wrapper&quot;).fadeOut(&quot;slow&quot;);
  });

  //wait for page submit, custom-made loading effect.
  document.addEventListener('submit', function() {
    ShowWaitingAnimation();
  });


  //////////////////////////////////////////////////////////////////////////////// Honeypot
  $('#HoneypotHiddenQuestion').prop('tabindex', -1);



//Create the button Valider - Promocode ( Quote adjustment questions )

function insertPromocodeButton() {
	if (Instanda.Variables.PageName == 'quote' &amp;&amp; (Instanda.Variables.PageNumber == 1 || Instanda.Variables.PageNumber == 5 )) {
		updateClone = $(&quot;.instanda-quote-update-button&quot;).clone();
		updateClone.attr(&quot;name&quot;, &quot;promocodeBtn&quot;);  
		updateClone.insertAfter(&quot;.promocode .questionItem &quot;);
		updateClone.attr(&quot;class&quot;, &quot;promocode&quot;);  
		var promo_info = $(&quot;.promocode .instanda-question-help-col&quot;);
		promo_info.hide();
		if ($(&quot;.alert.alert-danger.alert-dismissible.show&quot;).length > 0 &amp;&amp; Instanda.Variables.DeclineRule_Promo_PublicSite == &quot;NotDeclineRule&quot;) {
			updateClone.attr(&quot;class&quot;, &quot;promocode error-message&quot;);  
		}
		if ( Instanda.Variables.IsRenewal &amp;&amp; Instanda.Variables.Renewal_Promo == &quot;No&quot;) {
			 Instanda.Variables.PromocodePublic = &quot;&quot;;
		}

	}
}
insertPromocodeButton();


            
            
            
            
                if (!isTouchDevice) {
                    document.write('\x3Cscript type=&quot;text/javascript&quot; src=&quot;/Theme/js/plugins/bootstrap-miscellaneous/bootstrap-datetimepicker.min.js&quot;>\x3C/script>');
                }

                // Clear forms fields when then browser back button is used
                $(window).on(&quot;load&quot;, null, function () {

                    var $forms = $('form');

                    if ($forms.length !== 0) {

                        $forms.get(0).reset();
                    }
                });
            
            
            
                const clientOriginalShortDatePatternForDatePicker = 'dd/mm/yy';
                const clientOriginalShortDatePattern = 'DD/MM/YYYY';
                datepickerLogic(clientOriginalShortDatePatternForDatePicker, clientOriginalShortDatePattern, isTouchDevice, isResponsiveTheme)
            


    
        
            
                ×
                JavaScript Error
            
            
                ... error text
            
            
                Close
            
        
    

        





            
                

                Confidentialité &amp; Protection des données Conditions d’utilisation et Mentions Légales Politique de cookies

© 2021 Hiscox SA. All rights reserved by Hiscox SA.

                    Site built and hosted using
                    Instanda

    v1.84.0.5

                    insurance software.
                
            


        


    
        function PopUpReadOnlyToaster() {
            toastr.options = {
                &quot;closeButton&quot;: true,
			    &quot;debug&quot;: false,
			    &quot;newestOnTop&quot;: false,
			    &quot;progressBar&quot;: false,
			    &quot;positionClass&quot;: &quot;toast-bottom-full-width&quot;,
			    &quot;preventDuplicates&quot;: true,
			    &quot;onclick&quot;: null,
			    &quot;showDuration&quot;: &quot;300&quot;,
			    &quot;hideDuration&quot;: &quot;1000&quot;,
			    &quot;timeOut&quot;: &quot;5000&quot;,
			    &quot;extendedTimeOut&quot;: &quot;1000&quot;,
			    &quot;showEasing&quot;: &quot;swing&quot;,
			    &quot;hideEasing&quot;: &quot;linear&quot;,
			    &quot;showMethod&quot;: &quot;fadeIn&quot;,
			    &quot;hideMethod&quot;: &quot;fadeOut&quot;,
                &quot;escapeHtml&quot; : true
            }



            toastr[&quot;info&quot;]('This quote is displayed as readonly');
        }

            
            &quot;use strict&quot;;
			$(function () {
                var common = new Instanda.Common();
                var keeplLockOnPolicyInterval = window.setInterval(function ()

                    {
                    if (!common.KeepLockOnPolicy('Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7'))
                        clearInterval(keeplLockOnPolicyInterval);
                }, 60000);
            });

            


    

        
        
        
        
    






function OptanonWrapper(){window.dataLayer.push({event:&quot;OneTrustGroupsUpdated&quot;})};

(function(){var a=document.createElement(&quot;script&quot;);a.id=&quot;ws-diddyn-lib&quot;;a.type=&quot;text/javascript&quot;;a.async=!0;a.defer=!0;a.src=&quot;//cdn.wannaspeak.com/lib/did.js&quot;;var b=document.getElementsByTagName(&quot;script&quot;)[0];b.parentNode.insertBefore(a,b)})();
(function(c,d,f,g,e){c[e]=c[e]||[];var h=function(){var b={ti:&quot; 4010679&quot;};b.q=c[e];c[e]=new UET(b);c[e].push(&quot;pageLoad&quot;)};var a=d.createElement(f);a.src=g;a.async=1;a.onload=a.onreadystatechange=function(){var b=this.readyState;b&amp;&amp;&quot;loaded&quot;!==b&amp;&amp;&quot;complete&quot;!==b||(h(),a.onload=a.onreadystatechange=null)};d=d.getElementsByTagName(f)[0];d.parentNode.insertBefore(a,d)})(window,document,&quot;script&quot;,&quot;//bat.bing.com/bat.js&quot;,&quot;uetq&quot;);

  
  

var axel=Math.random()+&quot;&quot;,a=1E13*axel;document.write(&quot;\x3ciframe src\x3dhttps://6415832.fls.doubleclick.net/activityi;src\x3d6415832;type\x3dhpt5r0;cat\x3dhisco0;dc_lat\x3d;dc_rdid\x3d;tag_for_child_directed_treatment\x3d;tfua\x3d;npa\x3d;gdpr\x3d${GDPR};gdpr_consent\x3d${GDPR_CONSENT_755};ord\x3d1;num\x3d&quot;+a+'? width\x3d&quot;1&quot; height\x3d&quot;1&quot; frameborder\x3d&quot;0&quot; style\x3d&quot;display:none&quot;\x3e\x3c/iframe\x3e');


window._fs_host=&quot;eu1.fullstory.com&quot;;window._fs_script=&quot;edge.eu1.fullstory.com/s/fs.js&quot;;window._fs_org=&quot;o-18D5-eu1&quot;;window._fs_namespace=&quot;FS&quot;;
(function(e,k,l,m,h,f,a,d){l in e?e.console&amp;&amp;e.console.log&amp;&amp;e.console.log('FullStory namespace conflict. Please set window[&quot;_fs_namespace&quot;].'):(a=e[l]=function(c,b,g){a.q?a.q.push([c,b,g]):a._api(c,b,g)},a.q=[],f=k.createElement(m),f.async=1,f.crossOrigin=&quot;anonymous&quot;,f.src=&quot;https://&quot;+_fs_script,d=k.getElementsByTagName(m)[0],d.parentNode.insertBefore(f,d),a.identify=function(c,b,g){a(h,{uid:c},g);b&amp;&amp;a(h,b,g)},a.setUserVars=function(c,b){a(h,c,b)},a.event=function(c,b,g){a(&quot;event&quot;,{n:c,p:b},g)},a.anonymize=
function(){a.identify(!1)},a.shutdown=function(){a(&quot;rec&quot;,!1)},a.restart=function(){a(&quot;rec&quot;,!0)},a.log=function(c,b){a(&quot;log&quot;,[c,b])},a.consent=function(c){a(&quot;consent&quot;,!arguments.length||c)},a.identifyAccount=function(c,b){f=&quot;account&quot;;b=b||{};b.acctId=c;a(f,b)},a.clearUserCookie=function(){},a.setVars=function(c,b){a(&quot;setVars&quot;,[c,b])},a._w={},d=&quot;XMLHttpRequest&quot;,a._w[d]=e[d],d=&quot;fetch&quot;,a._w[d]=e[d],e[d]&amp;&amp;(e[d]=function(){return a._w[d].apply(this,arguments)}),a._v=&quot;1.3.0&quot;)})(window,document,window._fs_namespace,
&quot;script&quot;,&quot;user&quot;);




  window.teads_e=window.teads_e||[];window.teads_adv_id=19472;
Centre de préférences de la confidentialitéLorsque vous consultez un site Web, des données peuvent être stockées dans votre navigateur ou récupérées à partir de celui-ci, généralement sous la forme de cookies. Ces informations peuvent porter sur vous, sur vos préférences ou sur votre appareil et sont principalement utilisées pour s'assurer que le site Web fonctionne correctement. Les informations ne permettent généralement pas de vous identifier directement, mais peuvent vous permettre de bénéficier d'une expérience Web personnalisée. Parce que nous respectons votre droit à la vie privée, nous vous donnons la possibilité de ne pas autoriser certains types de cookies. Cliquez sur les différentes catégories pour obtenir plus de détails sur chacune d'entre elles, et modifier les paramètres par défaut. Toutefois, si vous bloquez certains types de cookies, votre expérience de navigation et les services que nous sommes en mesure de vous offrir peuvent être impactés. Pour modifier vos choix et préférences à tout moment, rendez-vous sur notre lien ‘politique de cookies’ :  
            https://www.hiscox.fr/politique-de-cookiesTout autoriserGérer les préférences de consentementCookies strictement nécessairesToujours actifIl s’agit des cookies indispensables au fonctionnement d’un site web. Sont inclus, par exemple, les cookies qui permettent aux utilisateurs de se connecter à des espaces sécurisés du site web, d’utiliser un panier d’achat ou d’utiliser les services de facturation électronique. Cliquez ici pour une liste d’exemples de cookies strictement nécessaires.Cookies d’analyse et de performance  Cookies d’analyse et de performance Ces cookies nous permettent de déterminer le nombre de visites et les sources du trafic sur notre site web, afin d'en mesurer et d’en améliorer les performances. Ils nous aident également à identifier les pages les plus / moins visitées et à évaluer comment les visiteurs naviguent sur le site. Toutes les informations, collectées par ces cookies, sont agrégées et donc anonymisées. Si vous n'acceptez pas cette catégorie de cookies, nous ne pourrons pas savoir quand vous avez réalisé votre visite sur notre site web.Cookies de fonctionalité  Cookies de fonctionalité Ils sont utilisés pour reconnaître les utilisateurs lorsqu'ils reviennent sur un site web. Ils permettent la personnalisation du contenu, la reconnaissance des utilisateurs et mémorisent les préférences des utilisateurs (par exemple, leur choix de langue ou de région). Cliquez ici pour une liste d’exemples de cookies de fonctionnalités.Cookies pour une publicité ciblée  Cookies pour une publicité ciblée Ces cookies peuvent être activés sur notre site web par nos partenaires publicitaires. Ils peuvent être utilisés par ces entreprises pour établir des profils sur vos intérêts, et afin de vous proposer des publicités ciblées sur d’autres sites. Ils fonctionnement uniquement en identifiant votre navigateur et votre appareil. Si vous n'acceptez pas cette catégorie de cookies, des publicités moins ciblées sur vos intérêts vous seront proposées lors de votre navigation sur d'autres sites web.Cookies de tiers  Cookies de tiers Lorsque vous visitez notre site web, vous pouvez remarquer des cookies qui ne nous sont pas liés. Lorsque vous consultez une page proposant du contenu intégré, par exemple sur YouTube, des cookies peuvent vous être envoyés à partir de ces sites web. Nous ne contrôlons pas la configuration de ces cookies et nous vous conseillons de consulter les sites web des tiers pour plus d'informations sur ces cookies et sur la façon de les gérer.

Pour désactiver toutes les publicités Hiscox ciblées sur les sites web d'autres sociétés, veuillez consulter http://www.youronlinechoices.com/fr/ , où vous pourrez désactiver les réseaux de publicité Microsoft, Amazon, Google, AppNexus, Turn, Facebook et Twitter. Veuillez noter qu'en désactivant les réseaux de publicité listés, vous désactiverez les publicités ciblées pour Hiscox et toute autre entreprise utilisant ces réseaux.

Hiscox utilise des fournisseurs qui peuvent placer des cookies sur le site web d’Hiscox afin de fournir divers services.Back ButtonListe des cookies Search IconFilter IconClear checkbox label labelApply CancelConsent Leg.Interest checkbox label label checkbox label label checkbox label label Confirmer la sélection(function(){var a=document.createElement(&quot;script&quot;);a.id=&quot;ws-diddyn-lib&quot;;a.type=&quot;text/javascript&quot;;a.async=!0;a.defer=!0;a.src=&quot;//cdn.wannaspeak.com/lib/did.js&quot;;var b=document.getElementsByTagName(&quot;script&quot;)[0];b.parentNode.insertBefore(a,b)})();
  
  

var axel=Math.random()+&quot;&quot;,a=1E13*axel;document.write(&quot;\x3ciframe src\x3dhttps://6415832.fls.doubleclick.net/activityi;src\x3d6415832;type\x3dhpt5r0;cat\x3dhisco0;dc_lat\x3d;dc_rdid\x3d;tag_for_child_directed_treatment\x3d;tfua\x3d;npa\x3d;gdpr\x3d${GDPR};gdpr_consent\x3d${GDPR_CONSENT_755};ord\x3d1;num\x3d&quot;+a+'? width\x3d&quot;1&quot; height\x3d&quot;1&quot; frameborder\x3d&quot;0&quot; style\x3d&quot;display:none&quot;\x3e\x3c/iframe\x3e');


window._fs_host=&quot;eu1.fullstory.com&quot;;window._fs_script=&quot;edge.eu1.fullstory.com/s/fs.js&quot;;window._fs_org=&quot;o-18D5-eu1&quot;;window._fs_namespace=&quot;FS&quot;;
(function(e,k,l,m,h,f,a,d){l in e?e.console&amp;&amp;e.console.log&amp;&amp;e.console.log('FullStory namespace conflict. Please set window[&quot;_fs_namespace&quot;].'):(a=e[l]=function(c,b,g){a.q?a.q.push([c,b,g]):a._api(c,b,g)},a.q=[],f=k.createElement(m),f.async=1,f.crossOrigin=&quot;anonymous&quot;,f.src=&quot;https://&quot;+_fs_script,d=k.getElementsByTagName(m)[0],d.parentNode.insertBefore(f,d),a.identify=function(c,b,g){a(h,{uid:c},g);b&amp;&amp;a(h,b,g)},a.setUserVars=function(c,b){a(h,c,b)},a.event=function(c,b,g){a(&quot;event&quot;,{n:c,p:b},g)},a.anonymize=
function(){a.identify(!1)},a.shutdown=function(){a(&quot;rec&quot;,!1)},a.restart=function(){a(&quot;rec&quot;,!0)},a.log=function(c,b){a(&quot;log&quot;,[c,b])},a.consent=function(c){a(&quot;consent&quot;,!arguments.length||c)},a.identifyAccount=function(c,b){f=&quot;account&quot;;b=b||{};b.acctId=c;a(f,b)},a.clearUserCookie=function(){},a.setVars=function(c,b){a(&quot;setVars&quot;,[c,b])},a._w={},d=&quot;XMLHttpRequest&quot;,a._w[d]=e[d],d=&quot;fetch&quot;,a._w[d]=e[d],e[d]&amp;&amp;(e[d]=function(){return a._w[d].apply(this,arguments)}),a._v=&quot;1.3.0&quot;)})(window,document,window._fs_namespace,
&quot;script&quot;,&quot;user&quot;);
  window.teads_e=window.teads_e||[];window.teads_adv_id=19472;
id(&quot;LastName--mdc&quot;)</value>
      <webElementGuid>bee2f48d-4024-4cb6-acdb-6cf3af32ee56</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;page-prequotequestions created-from-newbusiness page-number-1 package-12462 sale-stage-quickquote site-public theme-responsive safari&quot;]</value>
      <webElementGuid>56e41d5c-9bea-4c37-8a18-7263074d870c</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>9a3e6630-3c2b-4c5c-b651-70918803d874</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;


&lt;iframe src=&quot;https://www.googletagmanager.com/ns.html?id=GTM-NZRGST&quot;
height=&quot;0&quot; width=&quot;0&quot; style=&quot;display:none;visibility:hidden&quot;>&lt;/iframe>

        
        
        
Vous avez une question ? 01 89 07 75 97Appel gratuit | Du lundi au vendredi 08h30 - 20h00 hors jours fériés

    
        
            
                
                    Toggle navigation
                    
                    
                    
                

                    
                        
                        
                    
            
Votre profilVotre besoin Votre devis Vous êtes assuré


            

Vous avez une question ? 01 89 07 75 97Appel gratuit
Du lundi au vendredi 08h30 - 20h00
hors jours fériés



01 89 07 75 97Appel gratuit du lundi au vendredi 08h30 - 19h00 hors jours fériés


                

                                
Obtenir un devis                                
                                
Me connecter                                

                
            
        
    



                    







﻿﻿Estimez gratuitement le coût de votre assurance professionnelle

Votre devis Responsabilité Civile Professionnelle

Votre contrat Responsabilité Civile Professionnelle



    .navbar-static-top .nav {
        padding: 15px 0;
    }

    @media (max-width: 767px) {
        .navbar-brand {
            padding: 0;
        }

            .navbar-brand img {
                margin-top: 5px;
                margin-left: 13px;
            }
    }





×





                    
                        Pourriez-vous remplir les informations suivantes ?
                    

                                
                    


 

                    


            

                
                


	



                
                








        var $questionForm = null,
            $quoteContinueButton = null,
            newLocation = &quot;&quot;,
            redirect = &quot;False&quot; == &quot;True&quot;;

        if (redirect) {
            newLocation = &quot;/Public/Quote?PackageId=12462&amp;amp;quoteRef=Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&amp;amp;quoteIsReadOnly=False&amp;amp;fromNavLink=False&quot;;
            window.open(newLocation, &quot;_self&quot;);
        }

        $(function () {
            &quot;use strict&quot;;

            $quoteContinueButton = $(&quot;#continueButton&quot;);

            $questionForm = $quoteContinueButton
                .parentsUntil(&quot;form&quot;)
                .parent();

            if ($quoteContinueButton.length !== 0 &amp;&amp; $questionForm.length !== 0) {

                $quoteContinueButton.on(&quot;click&quot;, null, function () {

                    $questionForm
                        .find(&quot;#goBackward&quot;)
                        .remove()
                        .end()
                        .submit();
                });
            }

            ShowHelpTextOnFocus(&quot;WhenClicked&quot;);

            $(document.body).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.form-control[data-val-regex-pattern], .form-control[data-val-regexci-pattern]&quot; , &quot;'&quot; , &quot;, function () {
                var txtVal = $(this).val().trim();
                $(this).val(txtVal);
            });
        });




		
			
            



            
            
                



            


                    
                        Le preneur d&quot; , &quot;'&quot; , &quot;assurance
                    

            



            



        
            Votre nom
        
    
        




        

            
                
                    





                    
                        


                            
                            Madame
                        
                    
                    
                        


                            
                            Monsieur
                        
                    
            
                
            




    

       var func_Titre = (function(){

           var showOptionalText = false;

            if (showOptionalText) {

                var Titre = &quot; , &quot;'&quot; , &quot;Titre&quot; , &quot;'&quot; , &quot;;
                var dropdownadditionalchoiceinfo118957 = &quot; , &quot;'&quot; , &quot;dropdownadditionalchoiceinfo118957&quot; , &quot;'&quot; , &quot;;

                var extraInfoSelector = &quot;#&quot; + &quot; , &quot;'&quot; , &quot;dropdownadditionalchoiceinfo118957&quot; , &quot;'&quot; , &quot;;

                $(&quot;#&quot; + &quot; , &quot;'&quot; , &quot;Titre&quot; , &quot;'&quot; , &quot;).change(function(){
                    var lastchild = $(&quot;#&quot; + &quot; , &quot;'&quot; , &quot;Titre&quot; , &quot;'&quot; , &quot; + &quot; option:last-child&quot;).val();
                    toggleAdditionInfoTextBox(this,lastchild);
                });

                $(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;Titre&quot; , &quot;'&quot; , &quot;).click(function (){
                    var lastchild = $(&quot;.&quot; + &quot; , &quot;'&quot; , &quot;Titre&quot; , &quot;'&quot; , &quot; + &quot;_last&quot;).val();
                    toggleAdditionInfoTextBox(this,lastchild);
                });

                function toggleAdditionInfoTextBox(self,lastchildVal) {
                    var current = $(self).val();

                    if (lastchildVal === current) {
                        showAdditionInfoTextBox();
                    } else {
                        hideAdditionInfoTextBox();
                    }
                }

                function showAdditionInfoTextBox() {
                    appendToParent();
                    $(extraInfoSelector).show();
                    $(extraInfoSelector + &quot; INPUT&quot;).focus();
                }

                function hideAdditionInfoTextBox() {
                    appendToParent();
                    $(extraInfoSelector).hide();
                }

                function appendToParent() {
                    var parent = $(extraInfoSelector).parent();
                    $(parent).append($(extraInfoSelector));
                }

                return {
                    showAdditionInfoTextBox:showAdditionInfoTextBox,
                    hideAdditionInfoTextBox:hideAdditionInfoTextBox
                };
            }

           return {
               showAdditionInfoTextBox:{},
               hideAdditionInfoTextBox:{}
           };
       })();

        $(function () {
            var showOptionalText = false;
            var additionalInfoEntered = false;
            var lastQuestionWasSelected= false;
                    //or last value selected
            if (showOptionalText &amp;&amp; (additionalInfoEntered || lastQuestionWasSelected)) {
                func_Titre.showAdditionInfoTextBox();
            }
        });

        function autoCompleteShowAdditionInfo_Titre() {
            var showOptionalText = false;
            if (showOptionalText) {
                func_Titre.showAdditionInfoTextBox();
            }
        }

        function autoCompleteHideAdditionInfoTextBox_Titre() {
            var showOptionalText = false;
            if (showOptionalText) {
                func_Titre.hideAdditionInfoTextBox();
            }
        }

    

                
            

            
                
                    Nom
                
                

                    
                
      
        
        Nom
      
      
    
                
                    
                
            

            
                
                    Prénom
                
                

                    
                
      
        
        Prénom
      
      
    
                
                    
                
            

        

        

        
        
    
    
        
            
        
    
            



            



        
            Email professionnel
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Email professionnel
      
      
    
    
        
            
        
    
            



            



        
            Numéro de téléphone professionnel
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Numéro de téléphone professionnel
      
      
    
    
        
            
        
    
            
            
                



            


                    
                        Mon entreprise
                    

            



            



        
            Nom de l’entreprise
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Nom de l’entreprise
      
      
    
    
        
            
        
    
            



            



        
            Adresse de l’entreprise
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Adresse de l’entreprise
      
      
    
    
        
            
        
    
            



            



        
            Code postal de l&quot; , &quot;'&quot; , &quot;entreprise
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Code postal de l&quot; , &quot;'&quot; , &quot;entreprise
      
      
    
    
        
            
        
    
            



            



        
            Ville de l’entreprise
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Ville de l’entreprise
      
      
    
    
        
            
        
    
            



            








    


        

    

    
Je m’oppose à ce que mon numéro de téléphone et mon adresse email soient utilisés pour recevoir des offres commerciales d’Hiscox 




    


            
        

		

	
	
	
	
	
	


	

		
    Saved





	



                
                
                

		
			
				


    Saved


             Étape précédente






    var $questionForm = null,
        $quoteBackButton = null,
        $quoteGoBackwardElement = &quot;&lt;input type=\&quot;hidden\&quot; id=\&quot;goBackward\&quot; name=\&quot;goBackward\&quot; value=\&quot;false\&quot; data-ays-ignore=\&quot;true\&quot; />&quot;
        ;

    var backClickHandlerSet = false;

    $(function () {

        &quot;use strict&quot;;

        $quoteBackButton = $(&quot;#backButton&quot;);

        $questionForm = $quoteBackButton
            .parentsUntil(&quot;form&quot;)
            .parent();

        function clearIsDirty() {

            if ($questionForm.length !== 0) {

                $questionForm.remove(&quot;#goBackward&quot;);
            }
        }

        function setIsDirty () {

            if ($questionForm.length !== 0) {

                $questionForm
                    .append($quoteGoBackwardElement)
                    .find(&quot;#goBackward&quot;)
                    .val(true);
            }
        }

        if ($questionForm.length !== 0) {

            // Enable detection of changes to question answers
            $questionForm.areYouSure({ silent : true });
        }

        if ($quoteBackButton.length !== 0 &amp;&amp; $questionForm.length !== 0) {

            $questionForm.on(&quot;dirty.areYouSure&quot;, null, function () {

                setIsDirty();
                $questionForm.addClass(&quot;dirty&quot;);  // This should not be necessary, but there seems to be some unreliability when reading the property later
            });

            if (!backClickHandlerSet) {
                backClickHandlerSet = true;

                $quoteBackButton.on(&quot;click&quot;, null, function () {

                    var $questionForm = $(this)
                        .parentsUntil(&quot;form&quot;)
                        .parent();

                    $questionForm.trigger(&quot;checkform.areYouSure&quot;);

                    if ($questionForm.hasClass(&quot;dirty&quot;)) {
                        // Changes detected to the question answers.  Ensure client side validation is enforced.
                        //$questionForm.submit();
                        saveAnswersFrom(window.location.href, &quot; , &quot;'&quot; , &quot;quoteRef&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/Public/QuickQuoteQuestions?PackageId=12462&amp;quoteRef=Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&amp;createdFromType=NewBusiness&amp;isUpdate=False&amp;pageNumber=2&amp;goBackwards=True&amp;releaseDateOption=NotSet&amp;showSendReferralEmailButton=False&amp;quoteIsReadOnly=False&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SavedFrom&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SaveButton&quot; , &quot;'&quot; , &quot;);
                        // saveAnswers(window.location.href, &quot; , &quot;'&quot; , &quot;quoteRef&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/Public/QuickQuoteQuestions?PackageId=12462&amp;quoteRef=Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&amp;createdFromType=NewBusiness&amp;isUpdate=False&amp;pageNumber=2&amp;goBackwards=True&amp;releaseDateOption=NotSet&amp;showSendReferralEmailButton=False&amp;quoteIsReadOnly=False&quot; , &quot;'&quot; , &quot;);
                    } else {
                        // No changes were detected to the question answers.  Bypass client side validation and redirect to the previous page.
                        clearIsDirty();
                        location.href = &quot;/Public/QuickQuoteQuestions?PackageId=12462&amp;quoteRef=Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&amp;createdFromType=NewBusiness&amp;isUpdate=False&amp;pageNumber=2&amp;goBackwards=True&amp;releaseDateOption=NotSet&amp;showSendReferralEmailButton=False&amp;quoteIsReadOnly=False&quot;;
                    }
                });
            }
        }
    });


						

								Save
							Étape suivante 
						
				
			
		


                
                
                




                


    .toastr-btn {
        float: right;
        margin-right: 4px;
    }



    


    
        
            
                
                    Your session has expired, you will have to restart your quote
                
                
                    
                        Restart Quote
                    
                
            
        
    



    &quot;use strict&quot;;

    Date.prototype.getUTCTime = function () {
        return (this.getTime() + (this.getTimezoneOffset() * 60 * 60 * 1000)); // Add the offset hours in ms
    }

    Date.prototype.getUTC = function () {
        return new Date(this.getUTCTime());
    }

    Date.__proto__.utc = function () {
        return new Date().getUTCTime();
    }

    Date.__proto__.UTCNow = function () {
        return new Date(Date.utc());
    }

    const sessionWarningCookieName = &quot;__Host-SessionExpirationWarningTime&quot;;
    const minutesBeforeExpirationToWarn = 3;
    const minutesBeforeExpirationToWarnMili = minutesBeforeExpirationToWarn * 60000;
    const isEnabled = true;
    const sessionLengthMili = 900000;

    var sessionWarningTime = new Date();
    var sessionWarningTimer = 0;
    var sessionExpiredTimer = 0;

    var isLoginTypePublic = true

    $(function () {
        if (isEnabled === true) {
            InitialiseSessionAlerts();
        }
    });

    function SetSessionWarningTime(warningMilliseconds) {
        sessionWarningTime = new Date(Date.now() + warningMilliseconds);
    }

    function InitialiseSessionAlerts() {
        var milisecondsTillWarning = sessionLengthMili - minutesBeforeExpirationToWarnMili;

        if (!isLoginTypePublic || (window.location.href.includes(&quot;quoteRef&quot;) || window.location.href.includes(&quot;QuickQuoteQuestions&quot;) || window.location.href.includes(&quot;PreQuoteQuestionsContinue&quot;))) {
            ClearTimers();
            SetSessionWarningTime(milisecondsTillWarning);
            SetTimers(milisecondsTillWarning, sessionLengthMili);
        }
    }

    function AdjustSessionTimers(milisecondsTillWarning) {
        var milisecondsTillExpiration = milisecondsTillWarning + minutesBeforeExpirationToWarnMili;

        ClearTimers();
        SetSessionWarningTime(milisecondsTillWarning);
        SetTimers(milisecondsTillWarning, milisecondsTillExpiration);
    }

    function ClearTimers() {
        window.clearTimeout(sessionWarningTimer);
        window.clearTimeout(sessionExpiredTimer);
    }

    function SetTimers(warningWait, expirationWait) {
        if (warningWait > 0) {
            sessionWarningTimer = window.setTimeout(PromptKeepSessionActive, warningWait);
        }
        sessionExpiredTimer = window.setTimeout(AlertSessionExpired, expirationWait);

    }

    function CheckAndUpateTimers() {
        var validTimeChange = false;
        var dateString = getSessionExpiryFromCookie(sessionWarningCookieName);
        var expirationTime = new Date(dateString);
        var cookieWarningTime = new Date(expirationTime.getTime() - minutesBeforeExpirationToWarnMili);

        if (Date.now() &lt; expirationTime.getTime()) {
            if (cookieWarningTime.getTime() != sessionWarningTime.getTime()) {
                var timeDiff = Math.max(0, (cookieWarningTime.getTime() - Date.now()));

                // Cookie was changed by another tab, need to upate the timers in this tab
                if (timeDiff > 0) {
                    AdjustSessionTimers(timeDiff);
                    validTimeChange = true;
                }
            }
        }

        return validTimeChange;
    }

    function PromptKeepSessionActive() {
        toastr.remove();

        if (CheckAndUpateTimers()) {
            return;
        }

        var common = new Instanda.Common();
        if (isLoginTypePublic) {
            var message = &quot; , &quot;'&quot; , &quot;Your session is about to expire, do you wish to extend it?&quot; , &quot;'&quot; , &quot;;
            var buttonText = &quot; , &quot;'&quot; , &quot;Yes&quot; , &quot;'&quot; , &quot;;
        } else {
            var message = &quot; , &quot;'&quot; , &quot;Your session is about to expire, do you wish to extend it?&quot; , &quot;'&quot; , &quot;;
            var buttonText = &quot; , &quot;'&quot; , &quot;Yes&quot; , &quot;'&quot; , &quot;;
        }


        var encryptedQuoteRef = null;

             encryptedQuoteRef = &quot; , &quot;'&quot; , &quot;Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&quot; , &quot;'&quot; , &quot;;

        toastr.warning(&quot;&lt;div>&lt;button type=&quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot; id=&quot; , &quot;'&quot; , &quot;confirmationButtonYes&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;btn-extend-session btn instanda-button btn btn-primary toastr-btn&quot; , &quot;'&quot; , &quot;>&quot; + buttonText + &quot;&lt;/button>&lt;/div>&quot;, message,
            {
                positionClass: &quot;toast-top-center&quot;,
                timeOut: 0,
                extendedTimeOut: 0,
                preventDuplicates: false,
                closeButton: true,
                allowHtml: true,
                escapeHtml: false,
                onShown: function (toast) {
                    $(&quot;#confirmationButtonYes&quot;).click(function () {
                        common.keepSessionActive(encryptedQuoteRef);
                        InitialiseSessionAlerts();
                        toastr.remove();
                    });
                }
            });
    }

    function AlertSessionExpired() {
        toastr.remove();

        if (CheckAndUpateTimers()) {
            return;
        }

        if (isLoginTypePublic) {
            if (true) {
                $(&quot; , &quot;'&quot; , &quot;#publicLiveSessionExpiredPopupDialog&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
            } else {
                document.getElementById(&quot; , &quot;'&quot; , &quot;publicLiveSessionExpiredPopupDialog&quot; , &quot;'&quot; , &quot;).style.display = &quot;block&quot;;
            }
        } else {

            var message = &quot; , &quot;'&quot; , &quot;Your session has expired, you will be redirected to the login page&quot; , &quot;'&quot; , &quot;;
            var hideButtonText = &quot; , &quot;'&quot; , &quot;Hide&quot; , &quot;'&quot; , &quot;;
            var loginButtonText = &quot; , &quot;'&quot; , &quot;Login&quot; , &quot;'&quot; , &quot;;

            toastr.error(&quot;&lt;div>&lt;button type=&quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot; id=&quot; , &quot;'&quot; , &quot;confirmationButtonOK&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;btn instanda-button btn btn-primary toastr-btn&quot; , &quot;'&quot; , &quot;>&quot; + hideButtonText + &quot;&lt;/button>&quot; +
                &quot;&lt;button type=&quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot; id=&quot; , &quot;'&quot; , &quot;loginButtonOK&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;btn instanda-button btn btn-primary toastr-btn&quot; , &quot;'&quot; , &quot;>&quot; + loginButtonText + &quot;&lt;/button>&lt;/div>&quot;,
                message,
                {
                    positionClass: &quot;toast-top-center&quot;,
                    timeOut: 0,
                    extendedTimeOut: 0,
                    preventDuplicates: false,
                    closeButton: true,
                    allowHtml: true,
                    escapeHtml: false,
                    onShown: function (toast) {
                        $(&quot;#confirmationButtonOK&quot;).click(function () {
                            $(&quot;.toast-close-button&quot;).click();
                        });
                        $(&quot;#loginButtonOK&quot;).click(function () {
                            RedirectToLogin();
                        });
                    }
                });
        }

        killSession();
    }

    $(&quot;#publicLiveSessionExpiredRestartQuoteButton&quot;).click(function () {
        RestartQuote();
    });

    function RestartQuote() {
            window.location.href = &quot;/Public/QuickQuoteQuestions?packageId=12462&quot;;
    }

    function CreateCookie(name, value, days) {
        if (days) {
            var date = new Date();
            date.setTime(date.getTime() + (days * 24 * 60 * 60 * 1000));
            var expires = &quot;; expires=&quot; + date.toGMTString();
        } else {
            var expires = &quot;&quot;;
        }

        document.cookie = name + &quot;=&quot; + value + expires + &quot;; path=/&quot;;
    }

    function getSessionExpiryFromCookie(name) {
       const nameEQ = name + &quot;=&quot;;
        const ca = document.cookie.split(&quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot;);
        for (let i = 0; i &lt; ca.length; i++) {
            let c = ca[i];
            while (c.charAt(0) == &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;) c = c.substring(1, c.length);
            if (c.indexOf(nameEQ) == 0) {
                let value = c.substring(nameEQ.length, c.length);
                let expiry = new Date(value);
                return expiry;
            };
        }

        // If it can&quot; , &quot;'&quot; , &quot;t find the cookie then return current time (i.e. act like the session just expired)
        return Date.UTCNow();
    }

    function RedirectToLogin() {
        var loginType = &quot; , &quot;'&quot; , &quot;Public&quot; , &quot;'&quot; , &quot;;

        if (loginType == &quot;Client&quot;) {
            window.location.href = &quot;/&quot;;
            return;
        }

        $.ajax({
            type: &quot;GET&quot;,
            url: &quot;/Public/GetSessionLoginUrl?login=&quot; + loginType,
            success: function (data) {
                window.location.href = data;
                return;
            },
            error: function (jqXHR, textStatus, error) {

                var errorMessage = &quot;&quot;;

                if (jqXHR.responseJSON) {
                    if (jqXHR.responseJSON.IsRedirect) {
                        $(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;reinitialize.areYouSure&quot; , &quot;'&quot; , &quot;);
                        window.location.href = jqXHR.responseJSON.RedirectURL;
                        return;
                    } else if (jqXHR.responseJSON.HTMLContent) {
                        $(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;reinitialize.areYouSure&quot; , &quot;'&quot; , &quot;);
                        var htmlPageContent = jqXHR.responseJSON.HTMLContent;
                        document.write(htmlPageContent);
                        document.close();
                        return;
                    } else if (jqXHR.responseJSON.ErrorMessage) {
                        errorMessage = jqXHR.responseJSON.ErrorMessage;
                    } else {
                        errorMessage = jqXHR.responseJSON;
                    }
                } else {
                    errorMessage = JSON.parse(jqXHR.responseText);
                }
            }
        });
    }

    function killSession() {
        $.ajax({
            type: &quot;GET&quot;,
            url: &quot;/Public/AjaxPublicKillSession&quot;,
            success: function (data) { },
            error: function (jqXHR, textStatus, error) {

                var errorMessage = &quot;&quot;;

                if (jqXHR.responseJSON) {
                    if (jqXHR.responseJSON.IsRedirect) {
                       //the user will be redirected later based on the session expiration dialog choice
                        return;
                    } else if (jqXHR.responseJSON.HTMLContent) {
                        $(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;reinitialize.areYouSure&quot; , &quot;'&quot; , &quot;);
                        var htmlPageContent = jqXHR.responseJSON.HTMLContent;
                        document.write(htmlPageContent);
                        document.close();
                        return;
                    } else if (jqXHR.responseJSON.ErrorMessage) {
                        errorMessage = jqXHR.responseJSON.ErrorMessage;
                    } else {
                        errorMessage = jqXHR.responseJSON;
                    }
                } else if (jqXHR.status !== 403) {
                    errorMessage = jqXHR.responseText;
                }

                if (errorMessage !== &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    throw (errorMessage);
                }
            }
        });
    }

    $(document).ajaxSend(function (e, xhr, options) {

        var cookieTimeMili = new Date(getSessionExpiryFromCookie(sessionWarningCookieName)).getTime();
        var expiryTimeMili = cookieTimeMili + minutesBeforeExpirationToWarnMili;
        var currentTimeMili = new Date().getTime();

        var url = options.url;
        if (url != null &amp;&amp; url === &quot;/Public/AjaxPublicKillSession&quot;) {
            return;
        }

        if (expiryTimeMili &lt; currentTimeMili) {
            xhr.abort();
            $(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;reinitialize.areYouSure&quot; , &quot;'&quot; , &quot;);
            window.location.reload();
        }
    });

    $(document).ajaxComplete(function (event, jqXHR, ajaxOptions) {

        var extendSession = true;

        if (typeof jqXHR.statusText !== &quot;undefined&quot; &amp;&amp; jqXHR.statusText == &quot;abort&quot; &amp;&amp; typeof jqXHR.status !== &quot;undefined&quot; &amp;&amp; jqXHR.status == 0) {
            extendSession = false;
        } else if (typeof ajaxOptions.url !== &quot;undefined&quot; &amp;&amp; ajaxOptions.url === &quot;/Public/AjaxPublicKillSession&quot;) {
            extendSession = false;
        } else if (typeof jqXHR.responseJSON !== &quot;undefined&quot; &amp;&amp; typeof jqXHR.responseJSON.notExtendSession !== &quot;undefined&quot; &amp;&amp; typeof jqXHR.responseJSON.notExtendSession === &quot;boolean&quot;) {
            extendSession = !jqXHR.responseJSON.notExtendSession;
        }

        if (extendSession) {
            InitialiseSessionAlerts();
            //we only want to clear toast warnings
            $(&quot; , &quot;'&quot; , &quot;.btn-extend-session&quot; , &quot;'&quot; , &quot;).each(function (i, obj) {
                toastr.remove();
            });
        }
    });


            

            






Votre devis






Pour votre activité


Responsabilité Civile Professionnelle



Protection Juridique





Prime TTC 14,39 €






























Total TTC

14,39
par mois






Changer mon offre






€163,65TTC



€83,33TTC

soit 166,65 € par an



€42,41TTC

soit 169,64 € par an



€14,39TTC

soit 172,64 € par an

            
            




Les données personnelles fournies seront utilisées pour vous adresser le devis que vous sollicitez et pour la gestion de votre contrat d&quot; , &quot;'&quot; , &quot;assurance. Hiscox traite vos données personnelles avec le plus grand soin et ne les vend pas à des tiers à des fins de marketing. En cliquant sur &quot;Étape suivante&quot;, je déclare avoir pris connaissance de la notice d&quot; , &quot;'&quot; , &quot;information relative au traitement de mes données personnelles 










 

            
            
        



        
             
        
        

        

            $(function () {
                PreventDoubleSubmission($(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;));

            });

            window.thousandsSeperator = &quot; , &quot;'&quot; , &quot;&amp;#160;&quot; , &quot;'&quot; , &quot;;
            window.decimalSeperator = &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;;

            var validator = function () {

                var pub = {};

                pub.run = function (form) {
                    form.removeData(&quot; , &quot;'&quot; , &quot;validator&quot; , &quot;'&quot; , &quot;);
                    form.removeData(&quot; , &quot;'&quot; , &quot;unobtrusiveValidation&quot; , &quot;'&quot; , &quot;);
                    form.removeData(&quot; , &quot;'&quot; , &quot;submitted&quot; , &quot;'&quot; , &quot;);

                    $.validator.unobtrusive.parse(form);
                };

                return pub;
            }();

            $(&quot;form&quot;).submit(function (e) {

                var form = $(this);

                validator.run(form);
            });


        






  /* System hack */
  (function() {
    // hack system createAutoCompleteQuestion()
    const oldCreateAutoCompleteQuestion = createAutoCompleteQuestion;
    window.createAutoCompleteQuestion = function() {
      oldCreateAutoCompleteQuestion.apply(this, arguments);

      const questionId = arguments[0];
      const target = $(&quot;#&quot; + questionId).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-input&quot; , &quot;'&quot; , &quot;)[0];
      const observer = new MutationObserver(function(mutationList, observer) {
        for (const mutation of mutationList) {
          if (mutation.type == &quot; , &quot;'&quot; , &quot;childList&quot; , &quot;'&quot; , &quot; &amp;&amp; mutation.addedNodes.length > 0 &amp;&amp; mutation.addedNodes[0].classList.contains(&quot; , &quot;'&quot; , &quot;typeahead&quot; , &quot;'&quot; , &quot;)) {
            $(mutation.target).trigger(&quot; , &quot;'&quot; , &quot;autoComplete.built&quot; , &quot;'&quot; , &quot;);
          }
        }
      });
      observer.observe(target, {childList: true});
    }
  })();


  /* set tippy theme */
  tippy.setDefaultProps({
    &quot; , &quot;'&quot; , &quot;theme&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;custom&quot; , &quot;'&quot; , &quot;
  });


  /* Add class to body for style to fix input zoom for specific device or browser */
  navigator.userAgent.match(/iPhone|iPad|iPod/i) &amp;&amp; document.body.classList.add(&quot; , &quot;'&quot; , &quot;iOS&quot; , &quot;'&quot; , &quot;);
  navigator.userAgent.match(/Safari/i) &amp;&amp; document.body.classList.add(&quot; , &quot;'&quot; , &quot;safari&quot; , &quot;'&quot; , &quot;);


  /**
   * DOM Helpers - define html attributes for relocating elements
   *
   * Attribute names:
   *   - data-append-to
   *   - data-prepend-to
   *   - data-insert-after
   *   - data-insert-before
   *
   * Attribute values:
   *   valid jquery selector string
   */
  (function() {
    function domHelpers() {
      $(&quot; , &quot;'&quot; , &quot;[data-append-to]&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;append-to&quot; , &quot;'&quot; , &quot;).each(function() {
        var selector = $(this).data(&quot; , &quot;'&quot; , &quot;append-to&quot; , &quot;'&quot; , &quot;);
        $(this).appendTo(selector);
      });
      $(&quot; , &quot;'&quot; , &quot;[data-prepend-to]&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;prepend-to&quot; , &quot;'&quot; , &quot;).each(function() {
        var selector = $(this).data(&quot; , &quot;'&quot; , &quot;prepend-to&quot; , &quot;'&quot; , &quot;);
        $(this).prependTo(selector);
      });
      $(&quot; , &quot;'&quot; , &quot;[data-insert-after]&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;insert-after&quot; , &quot;'&quot; , &quot;).each(function() {
        var selector = $(this).data(&quot; , &quot;'&quot; , &quot;insert-after&quot; , &quot;'&quot; , &quot;);
        $(this).insertAfter(selector);
      });
      $(&quot; , &quot;'&quot; , &quot;[data-insert-before]&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;insert-before&quot; , &quot;'&quot; , &quot;).each(function() {
        var selector = $(this).data(&quot; , &quot;'&quot; , &quot;insert-before&quot; , &quot;'&quot; , &quot;);
        $(this).insertBefore(selector);
      });
      $(&quot; , &quot;'&quot; , &quot;[data-toggle-by-variable]&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;toggle-by-variable&quot; , &quot;'&quot; , &quot;).each(function() {
        var name = $(this).data(&quot; , &quot;'&quot; , &quot;toggle-by-variable&quot; , &quot;'&quot; , &quot;);
        $(this).toggle(Boolean(Instanda.Variables[name]));
      });
    }

    $(document)
      .ready(domHelpers)
      .ajaxComplete(domHelpers);
  })();


  /**
   * Util functions under namespace `Instanda.utils`
   *
   * isMobile()
   *   check if it is mobile
   */
  (function() {
    Instanda.utils = Instanda.utils || {};

    Instanda.utils.isMobile = function() {
      return window.outerWidth &lt;= 767;
    }
  })();


  // example: https://hiscoxdesign.instanda.com/Public/QuickQuoteQuestions?PackageId=12462&amp;pageNumber=1&amp;Sector_CHOICE=V%C3%A9hicule%20de%20Tourisme%20avec%20Chauffeur&amp;Turnover_NUM=2000&amp;Activity_CHOICE=Je%20ne%20trouve%20pas%20mon%20activit%C3%A9#debug
  (function() {
    $(window).one(&quot;load&quot;, function () {
      setTimeout(function() {
        const search = new URLSearchParams(window.location.search);
        for (const key of search.keys()) {
          const control = document.getElementById(key);
          if (control &amp;&amp; control.type !== &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) {
            const value = search.get(key);
            $(control).val(value).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);

            if (control.tagName === &quot; , &quot;'&quot; , &quot;SELECT&quot; , &quot;'&quot; , &quot;) {
              $(control).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-input&quot; , &quot;'&quot; , &quot;).one(&quot; , &quot;'&quot; , &quot;autoComplete.built&quot; , &quot;'&quot; , &quot;, function(event) {
                const $input = $(event.currentTarget).find(&quot; , &quot;'&quot; , &quot;.tt-input&quot; , &quot;'&quot; , &quot;);
                const controlId = $input.attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;);
                const value = search.get(controlId);
                $(event.currentTarget.children[0]).find(&quot; , &quot;'&quot; , &quot;.tt-input&quot; , &quot;'&quot; , &quot;).typeahead(&quot; , &quot;'&quot; , &quot;val&quot; , &quot;'&quot; , &quot;, value);
              });
            }
          }
        }
      }, 0);
    });
  }) ();


  /**
   * MDC Builders
   */
  (function() {
    // Templates
    var mdcSelectTempl = document.getElementById(&quot;mdc-select-tmpl&quot;).innerHTML;
    var mdcTextfieldTempl = document.getElementById(&quot;mdc-textfield-tmpl&quot;).innerHTML;
    var mdcSwitchTempl = document.getElementById(&quot;mdc-switch-tmpl&quot;).innerHTML;
    var mdcMenuTempl = document.getElementById(&quot;mdc-menu-tmpl&quot;).innerHTML;

    Mustache.parse(mdcSelectTempl);
    Mustache.parse(mdcTextfieldTempl);
    Mustache.parse(mdcSwitchTempl);
    Mustache.parse(mdcMenuTempl);

    // DEBUG
    function createDebugSwitch() {
      var rendered = Mustache.render(mdcSwitchTempl, {});
      var elem = $(rendered)[0];
      elem.style.cssText=&quot;position:fixed; bottom:15px; right:15px; display:block !important;&quot;;
      document.body.append(elem);

      var component = mdc.switchControl.MDCSwitch.attachTo(elem.querySelector(&quot; , &quot;'&quot; , &quot;.mdc-switch&quot; , &quot;'&quot; , &quot;));
      component.nativeControl_.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(event) {
        $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).toggleClass(&quot; , &quot;'&quot; , &quot;my-mdc-off&quot; , &quot;'&quot; , &quot;, event.target.checked);
      });
    }
    $(document).ready(function() {
      window.location.hash.indexOf(&quot; , &quot;'&quot; , &quot;debug&quot; , &quot;'&quot; , &quot;) > -1 &amp;&amp; createDebugSwitch();
    });

    // Classes
    class MDCComponentBuilder {
      constructor(ctrlElem, labelText, ctrlContainer, labelContainer, helpContainer = null) {
        this.ctrlElem = ctrlElem;
        this.labelText = this.htmlDecode(labelText);
        this.ctrlContainer = ctrlContainer;
        this.labelContainer = labelContainer;
        this.helpContainer = helpContainer;
        this.template = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        this.mutatedAttributes = [&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;];
      }

      htmlDecode(input) {
        var doc = new DOMParser().parseFromString(input, &quot;text/html&quot;);
        return doc.documentElement.textContent;
      }

      init() {
        this.mdcWrapperElement = null;
        this.mdcComponent = null;

        // @deprecated
        this.helpText = this.getHelpText();
      }

      // @deprecated
      getHelpText() {
        var $dom = $(this.ctrlContainer).find(&quot; , &quot;'&quot; , &quot;.field-validation-error&quot; , &quot;'&quot; , &quot;);
        return $dom.length > 0 ? $dom.html() : &quot;&quot;;
      }

      hideElements() {
        $(this.ctrlContainer).addClass(&quot; , &quot;'&quot; , &quot;my-mdc-hide&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;aria-hidden&quot; , &quot;'&quot; , &quot;, &quot;true&quot;);
        $(this.ctrlElem).attr(&quot; , &quot;'&quot; , &quot;tabindex&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;-1&quot; , &quot;'&quot; , &quot;);
        this.labelContainer &amp;&amp; $(this.labelContainer).addClass(&quot; , &quot;'&quot; , &quot;my-mdc-hide&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;aria-hidden&quot; , &quot;'&quot; , &quot;, &quot;true&quot;);
      }

      isBuilt() {
        return true &amp;&amp; $(this.ctrlContainer).data(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;);
      }

      build() {
        if (this.isBuilt()) return;

        this.prepareComponent();
        this.moveHelpIcon();
        this.syncStyle();
        this.bindEvents();
        this.observeMutation();
        this.hideElements();
        this.insert();
        $(this.ctrlContainer).data(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;, true);
        $(this.mdcWrapperElement).data(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;, true);
      }

      prepareComponent() {
      }

      moveHelpIcon() {
        const $iconContainer = $(this.ctrlContainer).find(&quot; , &quot;'&quot; , &quot;.instanda-responsive-help-icon-container&quot; , &quot;'&quot; , &quot;);
        $iconContainer.length > 0 &amp;&amp; $(this.mdcWrapperElement).append($iconContainer);
      }

      syncStyle() {
        this.mdcWrapperElement.setAttribute(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, this.ctrlElem.getAttribute(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;) || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
      }

      bindEvents() {
      }

      observeMutation() {
        var config = { childList: true, attributes: true, subtree: true };
        this.observer = new MutationObserver(function(mutationRecords, observer) {
          var syncStyle = false;
          var rebuild = false;
          mutationRecords.forEach(function(mutationRecord) {
            if (mutationRecord.type == &quot; , &quot;'&quot; , &quot;attributes&quot; , &quot;'&quot; , &quot;) {
              if (mutationRecord.attributeName == &quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;) syncStyle = true;
              if (this.mutatedAttributes.indexOf(mutationRecord.attributeName) > 0) rebuild = true;
            }
            if (mutationRecord.type == &quot; , &quot;'&quot; , &quot;childList&quot; , &quot;'&quot; , &quot;) {
              rebuild = true;
            }
          }.bind(this));

          if (syncStyle) this.syncStyle();
          if (rebuild) this.rebuild();
        }.bind(this))
        this.ctrlElem &amp;&amp; this.observer.observe(this.ctrlElem, config);
        this.helpContainer &amp;&amp; this.observer.observe(this.helpContainer, config);
      }

      destroy() {
        this.observer &amp;&amp; this.observer.disconnect();
        this.mdcComponent &amp;&amp; this.mdcComponent.destroy();
        $(this.mdcWrapperElement).removeData(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;).remove();
        $(this.ctrlContainer).removeData(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;);
      }

      revert() {
        this.destroy();
        $(this.ctrlContainer).removeClass(&quot; , &quot;'&quot; , &quot;my-mdc-hide&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;aria-hidden&quot; , &quot;'&quot; , &quot;);
        $(this.ctrlElem).removeAttr(&quot; , &quot;'&quot; , &quot;tabindex&quot; , &quot;'&quot; , &quot;);
        this.labelContainer &amp;&amp; $(this.labelContainer).removeClass(&quot; , &quot;'&quot; , &quot;my-mdc-hide&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;aria-hidden&quot; , &quot;'&quot; , &quot;, &quot;true&quot;);
      }

      rebuild() {
        this.destroy();
        this.init();
        this.build();
      }

      insert() {
        $(this.ctrlContainer).after(this.mdcWrapperElement);
      }
    }

    class MDCSwitchBuilder {
      constructor(ctrlElems, labelText, ctrlContainer, labelContainer, valueOn=&quot;Yes&quot;, valueOff=&quot;No&quot;) {
        this.ctrlElems = ctrlElems;
        this.labelText = labelText;
        this.ctrlContainer = ctrlContainer;
        this.labelContainer = labelContainer;
        // this.valueOn = valueOn;
        // this.valueOff = valueOff;
        this.ctrlElemOn = $(this.ctrlElems).filter(`[value=&quot;${valueOn}&quot;]`)[0];
        this.ctrlElemOff = $(this.ctrlElems).filter(`[value=&quot;${valueOff}&quot;]`)[0];

        this.template = mdcSwitchTempl;
        this.init();
      }

      init() {
        this.mdcWrapperElement = null;
        this.mdcComponent = null;
        this.observer = null;
      }

      hideElements() {
        $(this.ctrlContainer).addClass(&quot; , &quot;'&quot; , &quot;my-mdc-hide&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;aria-hidden&quot; , &quot;'&quot; , &quot;, &quot;true&quot;);
        $(this.ctrlElems).attr(&quot; , &quot;'&quot; , &quot;tabindex&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;-1&quot; , &quot;'&quot; , &quot;);
        this.labelContainer &amp;&amp; $(this.labelContainer).addClass(&quot; , &quot;'&quot; , &quot;my-mdc-hide&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;aria-hidden&quot; , &quot;'&quot; , &quot;, &quot;true&quot;);
      }

      isBuilt() {
        return true &amp;&amp; $(this.ctrlContainer).data(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;);
      }

      build() {
        if (this.isBuilt()) return;

        this.prepareComponent();
        this.bindEvents();
        this.observeMutation();
        this.hideElements();
        this.insert();
        $(this.ctrlContainer).data(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;, true);
        $(this.mdcWrapperElement).data(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;, true);
      }

      prepareComponent() {
        var data = {}
        data.label = this.labelText;
        data.disabled = (
          !this.ctrlElemOn.checked &amp;&amp; this.ctrlElemOn.disabled
        ) || (
          !this.ctrlElemOff.checked &amp;&amp; this.ctrlElemOff.disabled
        );
        data.checked = this.isChecked();
        var rendered = Mustache.render(this.template, data);
        this.mdcWrapperElement = $(rendered)[0];
        this.mdcComponent = mdc.switchControl.MDCSwitch.attachTo(this.mdcWrapperElement.querySelector(&quot; , &quot;'&quot; , &quot;.mdc-switch&quot; , &quot;'&quot; , &quot;));
      }

      isChecked() {
        return $(this.ctrlElemOn).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;);
      }

      bindEvents() {
        var builder = this;

        this.mdcComponent.nativeControl_.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, this._changeHandler.bind(this));

        $(this.ctrlElemOn)
          .on(&quot; , &quot;'&quot; , &quot;click.mdc&quot; , &quot;'&quot; , &quot;, function(event) {
            builder.mdcComponent.foundation_.setChecked(true);
          })

        $(this.ctrlElemOff)
          .on(&quot; , &quot;'&quot; , &quot;click.mdc&quot; , &quot;'&quot; , &quot;, function(event) {
            builder.mdcComponent.foundation_.setChecked(false);
          })
      }

      observeMutation() {
        var config = { childList: true, attributes: true, subtree: true };
        this.observer = new MutationObserver(function(mutationRecords, observer) {
          // var syncStyle = false;
          var rebuild = false;
          mutationRecords.forEach(function(mutationRecord) {
            if (mutationRecord.type == &quot; , &quot;'&quot; , &quot;attributes&quot; , &quot;'&quot; , &quot;) {
              // if (mutationRecord.attributeName != &quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;) syncStyle = true;
              if (mutationRecord.attributeName == &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) rebuild = true;
              if (mutationRecord.attributeName == &quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;) rebuild = true;
            }
            if (mutationRecord.type == &quot; , &quot;'&quot; , &quot;childList&quot; , &quot;'&quot; , &quot;) {
              rebuild = true;
            }
          });

          // if (syncStyle) this.syncStyle();
          if (rebuild) this.rebuild();
        }.bind(this))
        this.ctrlElemOn &amp;&amp; this.observer.observe(this.ctrlElemOn, config);
        this.ctrlElemOff &amp;&amp; this.observer.observe(this.ctrlElemOff, config);
      }

      insert() {
        $(this.ctrlContainer).after(this.mdcWrapperElement);
      }

      destroy() {
        this.mdcComponent.nativeControl_.removeEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, this._changeHandler.bind(this));
        $(this.ctrlElemOn).off(&quot; , &quot;'&quot; , &quot;click.mdc&quot; , &quot;'&quot; , &quot;);
        $(this.ctrlElemOff).off(&quot; , &quot;'&quot; , &quot;click.mdc&quot; , &quot;'&quot; , &quot;);
        this.mdcComponent.destroy();
        $(this.mdcWrapperElement).removeData(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;).remove();
        $(this.ctrlContainer).removeData(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;);
      }

      rebuild() {
        this.destroy();
        this.init();
        this.build();
      }

      _changeHandler(event) {
        if (event.target.checked) {
          $(this.ctrlElemOn).click();
        } else {
          $(this.ctrlElemOff).click();
        }
      }
    }

    class MDCTextFieldBuilder extends MDCComponentBuilder {
      constructor(ctrlElem, labelText, ctrlContainer, labelContainer, helpContainer) {
        super(ctrlElem, labelText, ctrlContainer, labelContainer, helpContainer);

        this.template = mdcTextfieldTempl;
        this.mutatedAttributes = [&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;];
        this.init();
      }

      prepareComponent() {
        var data = {}
        data.id = this.ctrlElem.id ? this.ctrlElem.id + &quot; , &quot;'&quot; , &quot;--mdc&quot; , &quot;'&quot; , &quot; : null;
        data.label = this.labelText;
        data.disabled = this.ctrlElem.disabled;
        data.readOnly = this.ctrlElem.readOnly;
        data.pattern = this.ctrlElem.getAttribute(&quot; , &quot;'&quot; , &quot;pattern&quot; , &quot;'&quot; , &quot;);
        data.autocomplete = this.ctrlElem.getAttribute(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;);
        data.value = this.ctrlElem.value;
        data.help = this.helpText;
        var rendered = Mustache.render(this.template, data);
        this.mdcWrapperElement = $(rendered)[0];
        this.mdcComponent = mdc.textField.MDCTextField.attachTo(this.mdcWrapperElement.querySelector(&quot; , &quot;'&quot; , &quot;.mdc-text-field&quot; , &quot;'&quot; , &quot;));
      }

      bindEvents() {
        var builder = this;
        var $input = $(this.ctrlElem);
        var $mdcInput = $(&quot;input&quot;, this.mdcWrapperElement)

        $mdcInput
          .on(&quot;input.mdc&quot;, function(event) {
            $input.val(event.target.value).trigger(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;);
          })
          .on(&quot;blur.mdc&quot;, function(event) {
            $input.trigger(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;);
          })
          .on(&quot;change.mdc&quot;, function(event) {
            $input.trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
          });

        $input
          .on(&quot;change.mdc&quot;, function(event) {
            builder.rebuild();
          })
          .on(&quot;mdc:rebuild&quot;, function(event) {
            builder.rebuild();
          })
      }

      destroy() {
        var $input = $(this.ctrlElem);
        var $mdcInput = $(&quot;input&quot;, this.mdcWrapperElement)

        $mdcInput
          .off(&quot;input.mdc&quot;)
          .off(&quot;blur.mdc&quot;);

        $input.off(&quot;change.mdc&quot;);

        super.destroy();
      }
    }

    class MDCSelectBuilder extends MDCComponentBuilder {
      constructor(ctrlElem, labelText, ctrlContainer, labelContainer) {
        super(ctrlElem, labelText, ctrlContainer, labelContainer);

        this.template = mdcSelectTempl;
        this.init();
      }

      init() {
        super.init();

        this.observer = null;
      }

      prepareComponent() {
        var data = {};
        data.id = this.ctrlElem.id ? this.ctrlElem.id + &quot; , &quot;'&quot; , &quot;--mdc&quot; , &quot;'&quot; , &quot; : null;
        data.label = this.labelText;
        data.selectedText = this.ctrlElem.selectedOptions[0] &amp;&amp; this.ctrlElem.selectedOptions[0].value;
        data.options = this.extractOptions();
        var rendered = Mustache.render(this.template, data);
        this.mdcWrapperElement = $(rendered)[0];
        this.mdcComponent = mdc.select.MDCSelect.attachTo(this.mdcWrapperElement.querySelector(&quot; , &quot;'&quot; , &quot;.mdc-select&quot; , &quot;'&quot; , &quot;));
      }

      bindEvents() {
        var builder = this;
        var $select = $(this.ctrlElem);

        this.mdcComponent.listen(&quot;MDCSelect:change&quot;, function(event) {
          builder.ctrlElem.selectedIndex = event.detail.index;
          builder.ctrlElem.dispatchEvent(new Event(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;));
        })

        $select
          .on(&quot;change.mdc&quot;, function(event) {
            builder.rebuild();
          });
      }

      extractOptions() {
        var options = [];
        $(this.ctrlElem).children(&quot; , &quot;'&quot; , &quot;option&quot; , &quot;'&quot; , &quot;).each(function(index, optionElem) {
          var data = {};
          data.selected = $(optionElem).is(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;);
          data.disabled = optionElem.disabled;
          data.hidden = optionElem.hidden;
          data.value = optionElem.value;
          data.text = data.value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; : $(optionElem).html();
          options.push(data);
        });
        return options;
      }
    }

    class mdcTypeAheadBuilder {
      constructor(ctrlElem, menuElem, labelText, ctrlContainer, labelContainer) {
        this.ctrlElem = ctrlElem;
        this.menuElem = menuElem;
        this.labelText = labelText;
        this.ctrlContainer = ctrlContainer;
        this.labelContainer = labelContainer;
      }

      prepareComponent() {
        var rendered = Mustache.render(mdcTextfieldTempl, {label: this.labelText});
        this.mdcWrapperElement = $(rendered)[0];
        this.mdcComponent = mdc.textField.MDCTextField.attachTo(this.mdcWrapperElement.querySelector(&quot; , &quot;'&quot; , &quot;.mdc-text-field&quot; , &quot;'&quot; , &quot;))

        var menuRendered = Mustache.render(mdcMenuTempl, {});
        this.mdcMenuElement =
        this.mdcMenuComponent = mdc.menu.MDCMenu.attachTo();
      }

      build() {
      }

      insert() {
      }
    }

    // building MDC component.
    var buildMDC = function(scope = document) {

      $(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.instanda-question-hierarchy > select&quot; , &quot;'&quot; , &quot;, scope).each(function(index, selectElem) {
        var $selectContainer = $(selectElem).parent();
        var $labelContainer = $selectContainer.prev(&quot; , &quot;'&quot; , &quot;.instanda-question-label&quot; , &quot;'&quot; , &quot;);
        if ($labelContainer.length === 0) {
          var $labelContainer = $selectContainer.closest(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).prev(&quot; , &quot;'&quot; , &quot;.instanda-question-label&quot; , &quot;'&quot; , &quot;);
        }
        var labelText = $labelContainer.children(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).html();
        var builder = new MDCSelectBuilder(selectElem, labelText, $selectContainer[0], $labelContainer[0]);
        builder.build();

        var observerConfig = { childList: true };
        var observer = new MutationObserver(function(mutationRecords, observer) {
          mutationRecords.forEach(function(mutationRecord) {
            if (mutationRecord.type == &quot; , &quot;'&quot; , &quot;childList&quot; , &quot;'&quot; , &quot;
            &amp;&amp; mutationRecord.removedNodes[0]
            &amp;&amp; mutationRecord.removedNodes[0].tagName == &quot; , &quot;'&quot; , &quot;SELECT&quot; , &quot;'&quot; , &quot;) {
              builder.revert();
              builder = null;
            }
          });
        });
        $selectContainer &amp;&amp; observer.observe($selectContainer[0], observerConfig);
      });

      $(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;)
        .not(&quot; , &quot;'&quot; , &quot;#question121029&quot; , &quot;'&quot; , &quot;)
        .find(&quot; , &quot;'&quot; , &quot;.form-group > .instanda-question-input&quot; , &quot;'&quot; , &quot;, scope)
        .children(&quot; , &quot;'&quot; , &quot;input[type=&quot;text&quot;], input[type=&quot;number&quot;], input[type=&quot;email&quot;], select&quot; , &quot;'&quot; , &quot;)
        .each(function(index, ctrlElem) {
        var $inputContainer = $(ctrlElem).parent();
        var $labelContainer = $inputContainer.siblings(&quot; , &quot;'&quot; , &quot;.instanda-question-inner-label&quot; , &quot;'&quot; , &quot;);
        if ($labelContainer.length == 0) {
          $labelContainer = $inputContainer.closest(&quot; , &quot;'&quot; , &quot;.questionItem > .instanda-text-question&quot; , &quot;'&quot; , &quot;).prev();
        }
        var labelText =
          ctrlElem.name == &quot; , &quot;'&quot; , &quot;AddressLine1&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;Adresse du local&quot; , &quot;'&quot; , &quot; :
          ctrlElem.name == &quot; , &quot;'&quot; , &quot;AddressLine2&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;Complément d’adresse&quot; , &quot;'&quot; , &quot; :
          ctrlElem.name == &quot; , &quot;'&quot; , &quot;Postcode&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;Code postal&quot; , &quot;'&quot; , &quot; :
          $labelContainer.children(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).html();
        var $helpContainer = $inputContainer.closest(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;[data-valmsg-for=&quot;&quot; , &quot;'&quot; , &quot;+ctrlElem.id+&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;);

        if ($(ctrlElem).is(&quot; , &quot;'&quot; , &quot;input[type=&quot;text&quot;], input[type=&quot;number&quot;], input[type=&quot;email&quot;]&quot; , &quot;'&quot; , &quot;)) {
          var builder = new MDCTextFieldBuilder(ctrlElem, labelText, $inputContainer[0], $labelContainer[0], $helpContainer[0]);
        }
        if ($(ctrlElem).is(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;)) {
          var builder = new MDCSelectBuilder(ctrlElem, labelText, $inputContainer[0], $labelContainer[0]);
        }
        builder.build();
      });

      $(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.instanda-number-input, .instanda-text-input&quot; , &quot;'&quot; , &quot;, scope).children(&quot; , &quot;'&quot; , &quot;input[type=&quot;text&quot;], input[type=&quot;number&quot;], input[type=&quot;email&quot;]&quot; , &quot;'&quot; , &quot;).each(function(index, ctrlElem) {
        var $inputContainer = $(ctrlElem).closest(&quot; , &quot;'&quot; , &quot;.questionItem > .instanda-text-question&quot; , &quot;'&quot; , &quot;);
        var $labelContainer = $inputContainer.prev();
        var labelText = $labelContainer.children(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).html();
        var $helpContainer = $(ctrlElem).closest(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;[data-valmsg-for=&quot;&quot; , &quot;'&quot; , &quot;+ctrlElem.id+&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;);

        var builder = new MDCTextFieldBuilder(ctrlElem, labelText, $inputContainer[0], $labelContainer[0], $helpContainer[0]);
        builder.build();
      });

      $(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;).
      not(&quot; , &quot;'&quot; , &quot;#question119074&quot; , &quot;'&quot; , &quot;).
      find(&quot; , &quot;'&quot; , &quot;.instanda-question-parent-yes-no&quot; , &quot;'&quot; , &quot;, scope).each(function(index, question) {
        var $ctrlContainer = $(question).find(&quot; , &quot;'&quot; , &quot;> .instanda-text-question&quot; , &quot;'&quot; , &quot;);
        var $labelContainer = $ctrlContainer.prev();
        var labelText = $labelContainer.children(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).html();
        var $ctrlElems = $ctrlContainer.find(&quot; , &quot;'&quot; , &quot;.radio-inline > input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;);
        // convert to switch control if 2 radios input found.
        if ($ctrlElems.length == 2) {
          var builder = new MDCSwitchBuilder($ctrlElems, labelText, $ctrlContainer[0], $labelContainer[0]);
          builder.build();
        }
      });

      var selectors = [
        &quot; , &quot;'&quot; , &quot;#question119728&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;#question119732&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;#question119735&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;#question119738&quot; , &quot;'&quot; , &quot;
      ];
      $(selectors.join(&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;)).each(function(index, coverElem) {
        // &lt;div id=&quot;&quot;>
        //  &lt;div class=&quot;instanda-questionHeader&quot;>&lt;h3>&lt;div>label&lt;/div>&lt;/h3>&lt;/div>
        //  &lt;div>
        //    &lt;input type=&quot;radio&quot; value=&quot;${on}&quot;>
        //    &lt;input type=&quot;radio&quot; value=&quot;${off}&quot;>
        //  &lt;/div>
        // &lt;/div>
        var $labelContainer = $(coverElem).children(&quot; , &quot;'&quot; , &quot;.instanda-questionHeader&quot; , &quot;'&quot; , &quot;)
        var $ctrlContainer = $labelContainer.next();
        var labelText = $labelContainer.find(&quot; , &quot;'&quot; , &quot;h3 > div&quot; , &quot;'&quot; , &quot;).html();
        var $ctrlElems = $ctrlContainer.find(&quot; , &quot;'&quot; , &quot;input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;);
        var builder = new MDCSwitchBuilder($ctrlElems, labelText, $ctrlContainer[0], $labelContainer[0], $ctrlElems[0].value, $ctrlElems[1].value);
        builder.build();
      });

      // $(&quot; , &quot;'&quot; , &quot;div&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;input[type=&quot;text&quot;], input[type=&quot;email&quot;]&quot; , &quot;'&quot; , &quot;).not(&quot; , &quot;'&quot; , &quot;[name*=&quot;OtherAnswer&quot;]&quot; , &quot;'&quot; , &quot;).not(&quot; , &quot;'&quot; , &quot;.focus-class-processed&quot; , &quot;'&quot; , &quot;).each(function() {
      //   var $input = $(this);

      //   $input.addClass(&quot; , &quot;'&quot; , &quot;focus-class-processed&quot; , &quot;'&quot; , &quot;);

      //   // -- HTML structure scenario 1
      //   var $label = $input.closest(&quot; , &quot;'&quot; , &quot;.instanda-question-input&quot; , &quot;'&quot; , &quot;).siblings(&quot; , &quot;'&quot; , &quot;div[class*=&quot;-label&quot;]&quot; , &quot;'&quot; , &quot;);

      //   // -- HTML structure scenario 2
      //   if ($label.length == 0) {
      //     $label = $input.parent().parent().parent(&quot; , &quot;'&quot; , &quot;.instanda-text-question.row&quot; , &quot;'&quot; , &quot;).siblings(&quot; , &quot;'&quot; , &quot;div[class*=&quot;-label&quot;]&quot; , &quot;'&quot; , &quot;)
      //   }

      // // -- HTML structure scenario 3
      //   if ($label.length == 0) {
      //     $label = $input.closest(&quot; , &quot;'&quot; , &quot;.instanda-text-question.row&quot; , &quot;'&quot; , &quot;).siblings(&quot; , &quot;'&quot; , &quot;div[class*=&quot;-label&quot;]&quot; , &quot;'&quot; , &quot;);
      //   }

      //   if ($label.length > 0) {
      //     processTextInput($input, $label);
      //   }
      // })
    }

    // hack system functions
    var oldAddMultiItemByClone = addMultiItemByClone;
    window.addMultiItemByClone = function() {
      oldAddMultiItemByClone.apply(this, arguments);

      var here = arguments[1];

      if (arguments[7] == &quot; , &quot;'&quot; , &quot;Activity_MI&quot; , &quot;'&quot; , &quot;) {
        // remove all wrapper have not been processed by builder.
        $(&quot; , &quot;'&quot; , &quot;#innerBody .mdc-component-wrapper&quot; , &quot;'&quot; , &quot;).each(function(index, wrapperElem) {
          if (!$(wrapperElem).data(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;)) {
            $(wrapperElem).remove();
          }
        })

        processItem();
      }

      // build the mdc for controls have not been processed by builder.
      $(document)
        .ready(buildMDC)
        .ajaxComplete(buildMDC);
    }

    function processItem() {
      $(&quot; , &quot;'&quot; , &quot;#insertActivity_MIMultiItemsHere > .instanda-multi-item&quot; , &quot;'&quot; , &quot;).each(function() {
        var thisItem = this;
        var clickHandler = function(event) {
          if (event.target.value == &quot;Yes&quot;) {
            $(thisItem).addClass(&quot; , &quot;'&quot; , &quot;manual-selected&quot; , &quot;'&quot; , &quot;);
          }
          if (event.target.value == &quot;No&quot;) {
            $(thisItem).removeClass(&quot; , &quot;'&quot; , &quot;manual-selected&quot; , &quot;'&quot; , &quot;);
          }
        };
        $(thisItem).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.radio-inline > input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;, clickHandler);
        $(thisItem).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.radio-inline > input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;, clickHandler);
      });
    }

    Instanda.mdc = Instanda.mdc || {};
    Instanda.mdc.MDCComponentBuilder = MDCComponentBuilder;
    Instanda.mdc.MDCTextFieldBuilder = MDCTextFieldBuilder;
    Instanda.mdc.MDCSelectBuilder = MDCSelectBuilder;
    Instanda.mdc.MDCSwitchBuilder = MDCSwitchBuilder;
    Instanda.mdc.build = buildMDC;
  })();


  /**
   * MDC Builders
   *
   * These builders will convert ordinary Instanda questions to material design component (MDC).
   * MDC script and HTML templates are loaded in &lt;head>.
   *
   * The interactions on MDC will sync to the cooresponding Instanda questions for system functions.
   * The builder will hide those Instanda questions after MDC built.
   *
   * Add `#debug` to the url will enable the toggle button at bottom right, it can toggle the Instanda questions
   * and MDC for debugging.
   *
   * This snippet provides a initial function `Instanda.mdc.build()`
   */
  (function() {
    // do nothing if there is no mdc builders
    if (!Instanda.mdc) return;

    $(document)
      .ready(Instanda.mdc.build)
      .ajaxComplete(function() { Instanda.mdc.build(document); });
  })();


  /**
   * Multi-step form
   *
   * It is a factory object for spliting form into multiple steps in one page
   */
  (function() {
    var multiStepForm = function(elem) {
      this._root = this._getElemFromArgument(elem);
      this._current = 0;
      this._forms = [];
    }
    multiStepForm.prototype.setBackBtn = function(elem) {
      var e = this._getElemFromArgument(elem);
      this.backBtn = e;
    };
    multiStepForm.prototype.setContinueBtn = function(elem) {
      var e = this._getElemFromArgument(elem);
      this.continueBtn = e;
    };
    multiStepForm.prototype.setPreviousBtn = function(elem, reference, method = &quot; , &quot;'&quot; , &quot;appendTo&quot; , &quot;'&quot; , &quot;) {
      var e = this._getElemFromArgument(elem);
      var ref = this._getElemFromArgument(reference);
      this.previousBtn = e;
      this.previousBtn &amp;&amp; ref &amp;&amp; this.previousBtn[method](ref);
    };
    multiStepForm.prototype.setNextBtn = function(elem, reference, method = &quot; , &quot;'&quot; , &quot;appendTo&quot; , &quot;'&quot; , &quot;) {
      var e = this._getElemFromArgument(elem);
      var ref = this._getElemFromArgument(reference);
      this.nextBtn = e;
      this.nextBtn &amp;&amp; ref &amp;&amp; this.nextBtn[method](ref);
    };
    multiStepForm.prototype.addForm = function(elem) {
      var e = this._getElemFromArgument(elem);
      e.length == 0 &amp;&amp; typeof elem == &quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot; &amp;&amp; console.warn(&quot; , &quot;'&quot; , &quot;MultiStepForm: cannot find form with selector &quot; , &quot;'&quot; , &quot; + elem);
      return e.length > 0 &amp;&amp; this._forms.push(e) || false;
    };
    multiStepForm.prototype.start = function() {
      // do nothing when no forms defined.
      if (this._forms.length == 0) {
        return;
      }

      var self = this;

      // display first form by default
      $.each(this._forms, function(index, form) {
        if (index > 0) {
          self._hideElem(form);
        }
      });

      self._bindEvents();
      self._displayButtons();
    };
    multiStepForm.prototype._getElemFromArgument = function(elem) {
      if (typeof elem === &quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;) {
        return $(elem);
      }
      if (elem instanceof jQuery) {
        return elem;
      }
      return null;
    }
    multiStepForm.prototype._hideElem = function(elem) {
      var e = this._getElemFromArgument(elem);
      e &amp;&amp; e.hide();
    };
    multiStepForm.prototype._showElem = function(elem) {
      var e = this._getElemFromArgument(elem);
      e &amp;&amp; e.show();
    };
    multiStepForm.prototype._bindEvents = function() {
      var self = this;
      self.nextBtn.on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function (e) { self._gotoNextForm() });
      self.previousBtn.on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function (e) { self._gotoPreviousForm() });
    };
    multiStepForm.prototype._gotoNextForm = function() {
      if (!this._isLastStep()) {
        this._hideElem(this._forms[this._current]);
        this._next();
        this._showElem(this._forms[this._current]);
        this._displayButtons();
      }
    };
    multiStepForm.prototype._gotoPreviousForm = function() {
      if (!this._isFirstStep()) {
        this._hideElem(this._forms[this._current]);
        this._previous();
        this._showElem(this._forms[this._current]);
        this._displayButtons();
      }
    };
    multiStepForm.prototype._isFirstStep = function() {
      return this._current == 0;
    };
    multiStepForm.prototype._isLastStep = function() {
      return this._current == this._forms.length - 1;
    };
    multiStepForm.prototype._next = function() {
      this._current++;
    };
    multiStepForm.prototype._previous = function() {
      this._current--;
    };
    multiStepForm.prototype._displayButtons = function() {
      if (this._isFirstStep()) {
        this._hideElem(this.previousBtn);
        this._showElem(this.backBtn);
      }
      if (!this._isFirstStep()) {
        this._showElem(this.previousBtn);
        this._hideElem(this.backBtn);
      }
      if (this._isLastStep()) {
        this._hideElem(this.nextBtn);
        this._showElem(this.continueBtn);
      }
      if (!this._isLastStep()) {
        this._showElem(this.nextBtn);
        this._hideElem(this.continueBtn);
      }
    };

    Instanda.multiStepForm = multiStepForm;
  })();


  /**
   * Create tooltip with help text content
   */
  (function() {
    var buildOverlay = function() {
      $(&quot; , &quot;'&quot; , &quot;.instanda-question-item&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;overlay-popup&quot; , &quot;'&quot; , &quot;).each(function(index, item) {
        var $icon = $(&quot; , &quot;'&quot; , &quot;.instanda-responsive-help-icon-container a&quot; , &quot;'&quot; , &quot;, item).removeAttr(&quot; , &quot;'&quot; , &quot;title data-target&quot; , &quot;'&quot; , &quot;);
        var $content = $(&quot; , &quot;'&quot; , &quot;.instanda-responsive-help-text > div&quot; , &quot;'&quot; , &quot;, item);

        if ($icon.length > 0 &amp;&amp; $content.length > 0) {
          var cls = $content[0].getAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;).replace(/col-\S*/g, &quot;&quot;).trim().replace(/\s+/g, &quot; &quot;);
          $content[0].setAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;, cls);
          tippy($icon[0], {
            content: $content[0]
          });
        }
      });
    };

    var construct = function() {
      buildOverlay();
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  $(document).ready(function () {
    //--- Show relevant cover options depending on trade
    if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quickquotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == 2) {
      switch(Instanda.Variables.TradeIdentifier_CHOICE) {
        case &quot; , &quot;'&quot; , &quot;PI + Office&quot; , &quot;'&quot; , &quot;:
          $(&quot; , &quot;'&quot; , &quot;input#InsuranceType_CHOICE_Voslocaux, input#InsuranceType_CHOICE_Votreactivitvoslocaux&quot; , &quot;'&quot; , &quot;).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).show();
          break;
        case &quot; , &quot;'&quot; , &quot;PI Only&quot; , &quot;'&quot; , &quot;:
          $(&quot; , &quot;'&quot; , &quot;input#InsuranceType_CHOICE_Voslocaux, input#InsuranceType_CHOICE_Votreactivitvoslocaux&quot; , &quot;'&quot; , &quot;).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).hide();
          break;
        default:
          $(&quot; , &quot;'&quot; , &quot;input#InsuranceType_CHOICE_Voslocaux, input#InsuranceType_CHOICE_Votreactivitvoslocaux&quot; , &quot;'&quot; , &quot;).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).hide();
      }
    }

    $(&quot; , &quot;'&quot; , &quot;header.navbar&quot; , &quot;'&quot; , &quot;).prependTo(&quot; , &quot;'&quot; , &quot;#innerBody&quot; , &quot;'&quot; , &quot;);
  }); // document.ready


  /**
   * Quick Quote Questions (Page 1)
   */
  (function() {
    // do nothing if it is not quick quote questions page 1.
    if ((Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;quickquotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;quickquotequestionscontinue&quot; , &quot;'&quot; , &quot;)
      || Instanda.Variables.PageNumber != 1) return;

    var createMultiStepForm = function() {
      if (Instanda.utils.isMobile()) {
        $(&quot; , &quot;'&quot; , &quot;#innerBody form&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;multiStepForm&quot; , &quot;'&quot; , &quot;).each(function() {
          var multiStep = new Instanda.multiStepForm();
          multiStep.addForm(&quot; , &quot;'&quot; , &quot;#question119038, #question120329&quot; , &quot;'&quot; , &quot;);
          multiStep.addForm(&quot; , &quot;'&quot; , &quot;#question118945&quot; , &quot;'&quot; , &quot;);
          multiStep.setContinueBtn(&quot; , &quot;'&quot; , &quot;[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;);
          multiStep.setBackBtn(&quot; , &quot;'&quot; , &quot;#backButton&quot; , &quot;'&quot; , &quot;);
          multiStep.setNextBtn(&quot; , &quot;'&quot; , &quot;&lt;a class=&quot;btn btn-primary&quot;>Étape suivante&lt;/a>&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;insertAfter&quot; , &quot;'&quot; , &quot;);
          multiStep.setPreviousBtn(&quot; , &quot;'&quot; , &quot;&lt;a class=&quot;btn btn-primary&quot;>Étape précédente&lt;/a>&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;pull-left&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;).insertBefore(multiStep.continueBtn.parent(&quot; , &quot;'&quot; , &quot;.pull-right&quot; , &quot;'&quot; , &quot;)), &quot; , &quot;'&quot; , &quot;appendTo&quot; , &quot;'&quot; , &quot;);
          multiStep.start();
        });
      }
    }

    // edit html for ease of style
    var editDom = function() {
      $(&quot; , &quot;'&quot; , &quot;#insertActivity_MIMultiItemsHere&quot; , &quot;'&quot; , &quot;).children(&quot; , &quot;'&quot; , &quot;.container&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;container&quot; , &quot;'&quot; , &quot;).children(&quot; , &quot;'&quot; , &quot;.row&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;);

      $(&quot; , &quot;'&quot; , &quot;#insertActivity_MIMultiItemsHere&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;[class^=&quot;col-&quot;], [class*=&quot; col-&quot;]&quot; , &quot;'&quot; , &quot;).each(function() {
        var cls = this.getAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;).replace(/col-\S*/g, &quot;&quot;).trim().replace(/\s+/g, &quot; &quot;);
        this.setAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;, cls);
      });

      $(&quot; , &quot;'&quot; , &quot;#multiItemSummary_119677&quot; , &quot;'&quot; , &quot;).children(&quot; , &quot;'&quot; , &quot;.container&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;container&quot; , &quot;'&quot; , &quot;).children(&quot; , &quot;'&quot; , &quot;.row&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;);

      $(&quot; , &quot;'&quot; , &quot;#multiItemSummary_119677&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;[class^=&quot;col-&quot;], [class*=&quot; col-&quot;]&quot; , &quot;'&quot; , &quot;).each(function() {
        var cls = this.getAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;).replace(/col-\S*/g, &quot;&quot;).trim().replace(/\s+/g, &quot; &quot;);
        this.setAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;, cls);
      });
    }

    // set min effective change date
    var setMinStartDate = function() {
      var $startDateInput = $(&quot; , &quot;'&quot; , &quot;#EffectiveChangeDate&quot; , &quot;'&quot; , &quot;);

      $startDateInput.length > 0 &amp;&amp; $startDateInput.once(&quot; , &quot;'&quot; , &quot;setMinStartDate&quot; , &quot;'&quot; , &quot;).each(function(elem) {
        var DatePicker = $startDateInput.parent(&quot; , &quot;'&quot; , &quot;.instanda-question-date&quot; , &quot;'&quot; , &quot;).data(&quot;DateTimePicker&quot;);

        if (DatePicker) {
          // bootstrap-datepicker
          DatePicker.minDate(moment().startOf(&quot; , &quot;'&quot; , &quot;d&quot; , &quot;'&quot; , &quot;));
        } else {
          // native
          $startDateInput[0].min = moment().startOf(&quot; , &quot;'&quot; , &quot;d&quot; , &quot;'&quot; , &quot;).format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;);
        }
      });
    }

    // Populate &quot;Je ne trouve pas mon activité&quot; when the checkbox is checked
    var populateCheckbox = function() {
      const $checkbox = $(&quot; , &quot;'&quot; , &quot;#MissingActivity_CONF&quot; , &quot;'&quot; , &quot;);
      const $sector = $(&quot; , &quot;'&quot; , &quot;#Sector_CHOICE&quot; , &quot;'&quot; , &quot;);
      const $activity = $(&quot; , &quot;'&quot; , &quot;#Activity_CHOICE&quot; , &quot;'&quot; , &quot;);

      const cannotFound = &quot; , &quot;'&quot; , &quot;Je ne trouve pas mon activité&quot; , &quot;'&quot; , &quot;;

      // bind autocomplete event after it built
      $(&quot; , &quot;'&quot; , &quot;#Activity_CHOICE&quot; , &quot;'&quot; , &quot;).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;autoComplete.built&quot; , &quot;'&quot; , &quot;, function(event) {
        $(event.currentTarget.children[0]).on(&quot; , &quot;'&quot; , &quot;typeahead:change typeahead:select&quot; , &quot;'&quot; , &quot;, function() {
          if ($(&quot; , &quot;'&quot; , &quot;.tt-input&quot; , &quot;'&quot; , &quot;, this).val() === cannotFound) {
            $checkbox.prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, true).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
          }
        })
      });
      // make activity field readonly when checkbox checked
      $checkbox.on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
        const $activity = $(&quot; , &quot;'&quot; , &quot;#Activity_CHOICE&quot; , &quot;'&quot; , &quot;);
        if ($(this).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
          $activity.val(cannotFound).trigger(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;readOnly&quot; , &quot;'&quot; , &quot;, true);
        } else {
          $activity.val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;readOnly&quot; , &quot;'&quot; , &quot;, false);
        }
      })
      $sector.on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
        $checkbox
          .prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false)
          .parent()
          .removeClass(&quot; , &quot;'&quot; , &quot;ticked&quot; , &quot;'&quot; , &quot;);
        $checkbox.closest(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;).toggle($(this).val() !== &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
      })

      $checkbox.closest(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;).toggle($sector.val() !== &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
    }

    var construct = function() {
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quickquotequestions&quot; , &quot;'&quot; , &quot;) {
        createMultiStepForm();
      }
      editDom();
      setMinStartDate();
      populateCheckbox();
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Quick Quote Questions (Page 2).
   */
  (function() {
    // do nothing if it is not quick quote questions page 2.
    if (Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;quickquotequestions&quot; , &quot;'&quot; , &quot; || Instanda.Variables.PageNumber != 2) return;

    var removeSelected = function() {
      $(&quot; , &quot;'&quot; , &quot;#question118955 .instanda-question-choice label.instanda-selected&quot; , &quot;'&quot; , &quot;).each(function(index, elem) {
        $(elem).removeClass(&quot; , &quot;'&quot; , &quot;instanda-selected&quot; , &quot;'&quot; , &quot;);
        var interval = setInterval(function() {
          var $input = $(&quot; , &quot;'&quot; , &quot;input:checked&quot; , &quot;'&quot; , &quot;, elem);
          if ($input.prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;)) {
            $(&quot; , &quot;'&quot; , &quot;input:checked&quot; , &quot;'&quot; , &quot;, elem).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
          }
          else {
            clearInterval(interval);
          }
        }, 400);
      });
    }

    var triggerContinue = function() {
      $(&quot; , &quot;'&quot; , &quot;#question118955 input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;triggerContinue&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
        $(&quot; , &quot;'&quot; , &quot;button[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;).click();
      })
    }

    var _removeChoices = function() {
      //-----Map activities to cover choices on quick quote questions page 2 - PI vs Office vs both
      var option_pi = $(&quot; , &quot;'&quot; , &quot;input#InsuranceType_CHOICE_Votreactivitprofessionnelle&quot; , &quot;'&quot; , &quot;).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-choice&quot; , &quot;'&quot; , &quot;);
      var option_office = $(&quot; , &quot;'&quot; , &quot;input#InsuranceType_CHOICE_Voslocaux&quot; , &quot;'&quot; , &quot;).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-choice&quot; , &quot;'&quot; , &quot;);
      var option_both = $(&quot; , &quot;'&quot; , &quot;input#InsuranceType_CHOICE_Votreactivitvoslocaux&quot; , &quot;'&quot; , &quot;).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-choice&quot; , &quot;'&quot; , &quot;);

      if ( Instanda.Variables.TradeIdentifier_CHOICE == &quot; , &quot;'&quot; , &quot;PI Only&quot; , &quot;'&quot; , &quot; ) {
        option_office.remove();
        option_both.remove();
      } else if ( Instanda.Variables.TradeIdentifier_CHOICE == &quot; , &quot;'&quot; , &quot;Office Only&quot; , &quot;'&quot; , &quot; ) {
        option_pi.remove();
        option_both.remove();
      }
    }

    var buildSwiper = function() {
      $(&quot; , &quot;'&quot; , &quot;#question118955&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;buildSwiper&quot; , &quot;'&quot; , &quot;).each(function(elem) {
        _removeChoices();

        var $slides = $(&quot; , &quot;'&quot; , &quot;.instanda-question-choice&quot; , &quot;'&quot; , &quot;, elem).addClass(&quot; , &quot;'&quot; , &quot;swiper-slide&quot; , &quot;'&quot; , &quot;);
        var $wrapper = $slides.parent().addClass(&quot; , &quot;'&quot; , &quot;swiper-wrapper&quot; , &quot;'&quot; , &quot;).wrap(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;my-swiper-container&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);
        $wrapper.parent().append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;swiper-button-prev&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);
        $wrapper.parent().append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;swiper-button-next&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);

        var swiperOption = {
          watchOverflow: true,
          slidesPerView: Instanda.utils.isMobile() ? 1 : $slides.length,
          navigation: {
            nextEl: &quot; , &quot;'&quot; , &quot;.swiper-button-next&quot; , &quot;'&quot; , &quot;,
            prevEl: &quot; , &quot;'&quot; , &quot;.swiper-button-prev&quot; , &quot;'&quot; , &quot;,
          },
        }
        var swiper = new Swiper(&quot; , &quot;'&quot; , &quot;.my-swiper-container&quot; , &quot;'&quot; , &quot;, swiperOption);
      });
    }

    var construct = function() {
      removeSelected();
      triggerContinue();
      buildSwiper();
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Quick Quote Questions (Page 3)
   */
  (function() {
    // do nothing if it is not quick quote questions page 3.
    if ((Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;quickquotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;quickquotequestionscontinue&quot; , &quot;'&quot; , &quot;)
      || Instanda.Variables.PageNumber != 3) return;

    var createMultiStepForm = function() {
      if (Instanda.utils.isMobile()) {
        $(&quot; , &quot;'&quot; , &quot;#innerBody form&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;multiStepForm&quot; , &quot;'&quot; , &quot;).each(function() {
          var multiStep = new Instanda.multiStepForm();
          multiStep.addForm(&quot; , &quot;'&quot; , &quot;.instanda-quick-quote-questions > .instanda-questionList > .instanda-well:nth-child(2)&quot; , &quot;'&quot; , &quot;);
          multiStep.addForm(&quot; , &quot;'&quot; , &quot;.instanda-quick-quote-questions > .instanda-questionList > .instanda-well:nth-child(3)&quot; , &quot;'&quot; , &quot;);
          multiStep.setContinueBtn(&quot; , &quot;'&quot; , &quot;[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;);
          multiStep.setBackBtn(&quot; , &quot;'&quot; , &quot;#backButton&quot; , &quot;'&quot; , &quot;);
          multiStep.setNextBtn(&quot; , &quot;'&quot; , &quot;&lt;a class=&quot;btn btn-primary&quot;>Étape suivante&lt;/a>&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;insertAfter&quot; , &quot;'&quot; , &quot;);
          multiStep.setPreviousBtn(&quot; , &quot;'&quot; , &quot;&lt;a class=&quot;btn btn-primary&quot;>Étape précédente&lt;/a>&quot; , &quot;'&quot; , &quot;, $(multiStep.continueBtn.parent(&quot; , &quot;'&quot; , &quot;.pull-right&quot; , &quot;'&quot; , &quot;)).siblings(&quot; , &quot;'&quot; , &quot;.pull-left&quot; , &quot;'&quot; , &quot;), &quot; , &quot;'&quot; , &quot;appendTo&quot; , &quot;'&quot; , &quot;);
          multiStep.start();
        });
      }
    }

    var addAutoComplete = function() {
      $(&quot; , &quot;'&quot; , &quot;#EnterpriseName_TXT&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;organization&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#AddressLine1&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;address-line1&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#AddressLine2&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;address-line2&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#City&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;address-level2&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#Postcode118934&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;postal-code&quot; , &quot;'&quot; , &quot;);
    }

    var construct = function() {
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quickquotequestions&quot; , &quot;'&quot; , &quot;) {
        createMultiStepForm();
      }
      addAutoComplete();
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Quick Quote Questions (Page 4)
   */
  (function() {
    if ((Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quickquotequestions&quot; , &quot;'&quot; , &quot; || Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quickquotequestionscontinue&quot; , &quot;'&quot; , &quot;)
    &amp;&amp; Instanda.Variables.PageNumber == 4) {

      // Fix the missing leading 0 on postcode field
      const fixPostCode = function() {
        const field = $(&quot; , &quot;'&quot; , &quot;#Postcode_FormB_NUM&quot; , &quot;'&quot; , &quot;);
        let value = field.val().replace(/\s+/,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
        if (value.match(/^\d+$/)) {
          value = value.padStart(5, 0);
        }
        field.val(value).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
      }

      // Auto fill activity choice to Form B
      const autoFillActivityChoice = function() {
        setTimeout(function() {
          $(&quot; , &quot;'&quot; , &quot;#Activity_CHOICE_FormB_TXT&quot; , &quot;'&quot; , &quot;).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;autoComplete.built&quot; , &quot;'&quot; , &quot;, function(event) {
            $(event.currentTarget.children[0]).find(&quot; , &quot;'&quot; , &quot;.tt-input&quot; , &quot;'&quot; , &quot;).typeahead(&quot; , &quot;'&quot; , &quot;val&quot; , &quot;'&quot; , &quot;, Instanda.Variables.Activity_CHOICE);
          });
          $(&quot; , &quot;'&quot; , &quot;#Sector_CHOICE_FormB_TXT&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.Sector_CHOICE).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
        }, 1000)
      }

      const populateTurnoverValue = function() {
        setTimeout(function() {
          $(&quot; , &quot;'&quot; , &quot;#Turnover_FormB_Num&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.Turnover_NUM).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
        }, 1000)
      }

      $(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, autoFillActivityChoice);
      $(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, populateTurnoverValue);
      $(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, fixPostCode);

      // $(document).ready(function() {
      //   autoFillActivityChoice();
      //   populateTurnoverValue();
      //   fixPostCode();
      // });
    }
  })();


  /**
   * Quick Quote (Page 1).
   */
  (function() {
    // do nothing if it is not quick quote page 1.
    if (Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;quickquote&quot; , &quot;'&quot; , &quot; || Instanda.Variables.PageNumber != 1) return;

    // Move radio button to each plan
    var moveRadioBtn = function() {
      $(&quot; , &quot;'&quot; , &quot;#PackSelect_Public_txt_Essential&quot; , &quot;'&quot; , &quot;).appendTo(&quot; , &quot;'&quot; , &quot;.cover-01 .cover-content&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#PackSelect_Public_txt_Equilibre&quot; , &quot;'&quot; , &quot;).appendTo(&quot; , &quot;'&quot; , &quot;.cover-02 .cover-content&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#PackSelect_Public_txt_Integral&quot; , &quot;'&quot; , &quot;).appendTo(&quot; , &quot;'&quot; , &quot;.cover-03 .cover-content&quot; , &quot;'&quot; , &quot;);
    }

    // disable continue button if no radio checked
    var disableContinueBtn = function() {
      var $radios = $(&quot; , &quot;'&quot; , &quot;.cover input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;);
      var $btn = $(&quot; , &quot;'&quot; , &quot;#continueButton&quot; , &quot;'&quot; , &quot;);
      var hasChecked = false;
      if ($btn.length > 0) {
        $radios.each(function(index, radio) {
          hasChecked = hasChecked || radio.checked;
        });
        $btn[0].disabled = !hasChecked;
      }
    }

// sync items across covers in 3 boxes
    var syncCoverItems = function() {
      var $list = $(&quot; , &quot;'&quot; , &quot;.cover-list&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;syncCoverItems&quot; , &quot;'&quot; , &quot;);
      if ($list.length > 0 &amp;&amp; !(Instanda.Variables.Sector_CHOICE == &quot; , &quot;'&quot; , &quot;Informatique&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.Turnover_NUM &lt;= 95000 &amp;&amp; Instanda.Variables.InsuranceType_CHOICE == &quot; , &quot;'&quot; , &quot;Votre activité professionnelle&quot; , &quot;'&quot; , &quot;)){
        var $covers = $(&quot; , &quot;'&quot; , &quot;.cover-wrapper&quot; , &quot;'&quot; , &quot;, $list);
        var lastIndex = $covers.length - 1;

	//Commented by (DanielPeresFR ) - APFD-1233 and APFD-1234

       /* var removeLastCover =
          (Instanda.Variables.Sector_CHOICE === &quot; , &quot;'&quot; , &quot;Véhicule de Tourisme avec Chauffeur&quot; , &quot;'&quot; , &quot;) ||
          (Instanda.Variables.Sector_CHOICE === &quot; , &quot;'&quot; , &quot;Artisans, Commerçants et e-Commerçants de détail&quot; , &quot;'&quot; , &quot;
            &amp;&amp; Instanda.Variables.InsuranceType_CHOICE === &quot; , &quot;'&quot; , &quot;Votre activité professionnelle&quot; , &quot;'&quot; , &quot;);

        if (removeLastCover) {
          $covers.last().remove();
          var lastIndex = $covers.length - 2;
        }*/

        var $lastCover = $covers[lastIndex];
        $(&quot; , &quot;'&quot; , &quot;.cover-content > div&quot; , &quot;'&quot; , &quot;, $lastCover).each(function(itemIndex, item) {
          $covers.slice(0, lastIndex).each(function(coverIndex, cover) {
            const $coverItems = $(&quot; , &quot;'&quot; , &quot;.cover-content > div&quot; , &quot;'&quot; , &quot;, cover)
            const length = $coverItems.length;
            if (itemIndex >= length) {
              $(&quot; , &quot;'&quot; , &quot;.cover-content input&quot; , &quot;'&quot; , &quot;, cover).before($(item).clone().addClass(&quot; , &quot;'&quot; , &quot;grey-out&quot; , &quot;'&quot; , &quot;));
            }
          });
        });
      }
    }

    var toggleSelectedClass = function() {
      const toggleClass = function(radio, box) {
        var checked = $(radio).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;);
        if (checked) {
          $(box).addClass(&quot; , &quot;'&quot; , &quot;instanda-selected&quot; , &quot;'&quot; , &quot;)
          $(boxes).not(box).removeClass(&quot; , &quot;'&quot; , &quot;instanda-selected&quot; , &quot;'&quot; , &quot;);
        } else {
          $(box).removeClass(&quot; , &quot;'&quot; , &quot;instanda-selected&quot; , &quot;'&quot; , &quot;);
        }
      }
      const boxes = $(&quot; , &quot;'&quot; , &quot;.cover-wrapper > .box&quot; , &quot;'&quot; , &quot;);

      boxes
        .each(function(index, box) {
          var $radio = $(&quot; , &quot;'&quot; , &quot;input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;, box)
          toggleClass($radio, box, boxes);
        })
        .once(&quot; , &quot;'&quot; , &quot;coverWrapperClick&quot; , &quot;'&quot; , &quot;)
        .on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(event) {
          const box = event.currentTarget;
          event.stopPropagation();
          var $radio = $(&quot; , &quot;'&quot; , &quot;input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;, box);
          $radio.prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, true).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
          toggleClass($radio, box, boxes);
        });
    }

    var buildSwiper = function() {
      $(&quot; , &quot;'&quot; , &quot;#instanda-quote-content&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;buildSwiper&quot; , &quot;'&quot; , &quot;).each(function(elem) {
        var $slides = $(&quot; , &quot;'&quot; , &quot;.cover-list > .cover-wrapper&quot; , &quot;'&quot; , &quot;, elem).addClass(&quot; , &quot;'&quot; , &quot;swiper-slide&quot; , &quot;'&quot; , &quot;);
        var $wrapper = $slides.parent().addClass(&quot; , &quot;'&quot; , &quot;swiper-wrapper&quot; , &quot;'&quot; , &quot;).wrap(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;my-swiper-container&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);
        $wrapper.parent().append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;swiper-button-prev&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);
        $wrapper.parent().append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;swiper-button-next&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);

        var swiperOption = {
          watchOverflow: true,
          slidesPerView: Instanda.utils.isMobile() ? 1 : $slides.length,
          navigation: {
            nextEl: &quot; , &quot;'&quot; , &quot;.swiper-button-next&quot; , &quot;'&quot; , &quot;,
            prevEl: &quot; , &quot;'&quot; , &quot;.swiper-button-prev&quot; , &quot;'&quot; , &quot;,
          }
        }
        var swiper = new Swiper(&quot; , &quot;'&quot; , &quot;.my-swiper-container&quot; , &quot;'&quot; , &quot;, swiperOption);
        swiper.slideTo(1);
      });
    }

    var buildToolTips = function() {
      $(&quot; , &quot;'&quot; , &quot;.help-text&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;overlay-popup&quot; , &quot;'&quot; , &quot;).each(function(index, item) {
        var $icon = $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, item);
        var $content = $(&quot; , &quot;'&quot; , &quot;&lt;div>&quot; , &quot;'&quot; , &quot; + $icon.data(&quot; , &quot;'&quot; , &quot;content&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;);

        if ($icon.length > 0 &amp;&amp; $content.length > 0) {
          tippy($icon[0], {
            content: $content[0]
          });
        }
      });
    };

    var construct = function() {
      moveRadioBtn();
      syncCoverItems();
      toggleSelectedClass();
      buildSwiper();
      buildToolTips();
      disableContinueBtn();
    }

    $(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function() {
      setTimeout(function() {
        toggleSelectedClass();
      }, 0);
    });
    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Pre Quote Questions (Page 1).
   */
  (function() {
    // do nothing if it is not pre quote questions page 1.
    if ((Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;prequotequestions&quot; , &quot;'&quot; , &quot;
      &amp;&amp; Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;prequotequestionscontinue&quot; , &quot;'&quot; , &quot;)
      || Instanda.Variables.PageNumber != 1) return;

    var createMultiStepForm = function() {
      if (Instanda.utils.isMobile()) {
        $(&quot; , &quot;'&quot; , &quot;#innerBody form&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;multiStepForm&quot; , &quot;'&quot; , &quot;).each(function() {
          var multiStep = new Instanda.multiStepForm();
          multiStep.addForm(&quot; , &quot;'&quot; , &quot;.instanda-pre-quote-questions > .instanda-questionList > .instanda-well:nth-child(2)&quot; , &quot;'&quot; , &quot;);
          multiStep.addForm(&quot; , &quot;'&quot; , &quot;.instanda-pre-quote-questions > .instanda-questionList > .instanda-well:nth-child(3)&quot; , &quot;'&quot; , &quot;);
          multiStep.setContinueBtn(&quot; , &quot;'&quot; , &quot;[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;);
          multiStep.setBackBtn(&quot; , &quot;'&quot; , &quot;#backButton&quot; , &quot;'&quot; , &quot;);
          multiStep.setNextBtn(&quot; , &quot;'&quot; , &quot;&lt;a class=&quot;btn btn-primary&quot;>Étape suivante&lt;/a>&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;insertAfter&quot; , &quot;'&quot; , &quot;);
          multiStep.setPreviousBtn(&quot; , &quot;'&quot; , &quot;&lt;a class=&quot;btn btn-primary&quot;>Étape précédente&lt;/a>&quot; , &quot;'&quot; , &quot;, $(multiStep.continueBtn.parent(&quot; , &quot;'&quot; , &quot;.pull-right&quot; , &quot;'&quot; , &quot;)).siblings(&quot; , &quot;'&quot; , &quot;.pull-left&quot; , &quot;'&quot; , &quot;), &quot; , &quot;'&quot; , &quot;appendTo&quot; , &quot;'&quot; , &quot;);
          multiStep.start();
        });
      }
    }

    // pre-fill address
    var autofillAddress = function() {
      setTimeout(function() {
        if (Instanda.Variables.RiskAddress_YN === &quot;Confirmed&quot;) {
          $(&quot; , &quot;'&quot; , &quot;#CompanyName_TXT&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.EnterpriseName_TXT);
          $(&quot; , &quot;'&quot; , &quot;#CompanyName_TXT--mdc&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.EnterpriseName_TXT).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
          $(&quot; , &quot;'&quot; , &quot;#CompanyAddressLine_TXT&quot; , &quot;'&quot; , &quot;).val([Instanda.Variables.AddressLine1,Instanda.Variables.AddressLine2].join(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;).trim());
          $(&quot; , &quot;'&quot; , &quot;#CompanyAddressLine_TXT--mdc&quot; , &quot;'&quot; , &quot;).val([Instanda.Variables.AddressLine1,Instanda.Variables.AddressLine2].join(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;).trim()).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
          $(&quot; , &quot;'&quot; , &quot;#CompanyPostcode_TXT&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.Postcode.toString().padStart(5, &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;));
          $(&quot; , &quot;'&quot; , &quot;#CompanyPostcode_TXT--mdc&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.Postcode.toString().padStart(5, &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;)).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
          $(&quot; , &quot;'&quot; , &quot;#CompanyCity_TXT&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.City);
          $(&quot; , &quot;'&quot; , &quot;#CompanyCity_TXT--mdc&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.City).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
        }
      }, 1000);
    }

    // set autocomplete attributes
    var addAutoComplete = function() {
      $(&quot; , &quot;'&quot; , &quot;#FirstName&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;given-name&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#LastName&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;family-name&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#Email_TXT&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#TelephoneNumber_NUM&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tel&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#CompanyName_TXT&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;organization&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#CompanyAddressLine_TXT&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;street-address&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#CompanyPostcode_TXT&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;postal-code&quot; , &quot;'&quot; , &quot;);
    }

    var construct = function() {
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;prequotequestions&quot; , &quot;'&quot; , &quot;) {
        createMultiStepForm();
      }
      addAutoComplete();
    }

    $(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, autofillAddress);

    $(document)
      .ready(function() {
        construct();
      })
      .ajaxComplete(construct);
  })();


  /**
   * Pre Quote Questions (Page 2).
   */
  (function() {
    // do nothing if it is not pre quote questions page 2.
    if (Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;prequotequestions&quot; , &quot;'&quot; , &quot; || Instanda.Variables.PageNumber != 2) return;

    // remove bootstrap grid class for ease of style
    var removeColClass = function() {
      $(&quot; , &quot;'&quot; , &quot;.instanda-main-content .questionList&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;[class^=&quot;col-&quot;], [class*=&quot; col-&quot;]&quot; , &quot;'&quot; , &quot;).each(function() {
        var cls = this.getAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;).replace(/col-\S*/g, &quot;&quot;).trim().replace(/\s+/g, &quot; &quot;);
        this.setAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;, cls);
      });
    }

    // move tool tip icon
    var moveIcon = function() {
      $(&quot; , &quot;'&quot; , &quot;.instanda-main-content .questionList&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.instanda-question-help-col&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;moveIcon&quot; , &quot;'&quot; , &quot;).each(function() {
        var $iconContainer = $(this);
        $iconContainer.parent().parent().append($iconContainer);
      });
    }

    var construct = function() {
      removeColClass();
      moveIcon();
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Pre Quote Questions (Page 3).
   */
  (function() {
    // do nothing if it is not pre quote questions page 3.
    if (Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;prequotequestions&quot; , &quot;'&quot; , &quot; || Instanda.Variables.PageNumber != 3) return;

    // set min policy start date
    var setMinStartDate = function() {
      var $continueBtn = $(&quot; , &quot;'&quot; , &quot;[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;);
      var $startDateInput = $(&quot; , &quot;'&quot; , &quot;#PolicyStartDate_DATE&quot; , &quot;'&quot; , &quot;);

      $startDateInput.once(&quot; , &quot;'&quot; , &quot;setMinStartDate&quot; , &quot;'&quot; , &quot;).each(function(elem) {
        var DatePicker = $startDateInput.parent(&quot; , &quot;'&quot; , &quot;.instanda-question-date&quot; , &quot;'&quot; , &quot;).data(&quot;DateTimePicker&quot;);

        if (DatePicker) {
          // bootstrap-datepicker
          DatePicker.minDate(moment().startOf(&quot; , &quot;'&quot; , &quot;d&quot; , &quot;'&quot; , &quot;));
        } else {
          // native
          $startDateInput[0].min = moment().startOf(&quot; , &quot;'&quot; , &quot;d&quot; , &quot;'&quot; , &quot;).format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;);
        }
        $startDateInput.on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, function() {
          $continueBtn.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
        }).on(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;, function() {
          $continueBtn.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
        });
      });
    }

    var construct = function() {
      setMinStartDate()
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Move &quot;instanda-well&quot; help text banner
   */
  (function() {
    var moved = false;

    var _moveBanner = function(target) {
      const $banner = $(&quot; , &quot;'&quot; , &quot;.instanda-main-content&quot; , &quot;'&quot; , &quot;).find(target);
      if (!moved &amp;&amp; $banner.length > 0) {
        $banner.prependTo(&quot; , &quot;'&quot; , &quot;.instanda-main-content-container&quot; , &quot;'&quot; , &quot;);
        moved = true;
      } else {
        $banner.remove();
      }
    }

    var moveBanner = function() {
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;prequotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == 1) {
        _moveBanner(&quot; , &quot;'&quot; , &quot;#question118956&quot; , &quot;'&quot; , &quot;);
      }
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;prequotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == 2) {
        _moveBanner(&quot; , &quot;'&quot; , &quot;#question120226&quot; , &quot;'&quot; , &quot;);
      }
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;prequotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == 3) {
        _moveBanner(&quot; , &quot;'&quot; , &quot;#question120292&quot; , &quot;'&quot; , &quot;);
      }
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quickquote&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == 1) {
        _moveBanner(&quot; , &quot;'&quot; , &quot;#instanda-quote-content .questionList&quot; , &quot;'&quot; , &quot;);
      }
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;postquotequestions&quot; , &quot;'&quot; , &quot;) {
        _moveBanner(&quot; , &quot;'&quot; , &quot;#question120291&quot; , &quot;'&quot; , &quot;);
        _moveBanner(&quot; , &quot;'&quot; , &quot;#question119073&quot; , &quot;'&quot; , &quot;);
        _moveBanner(&quot; , &quot;'&quot; , &quot;#question120358&quot; , &quot;'&quot; , &quot;);
      }
    }

    $(document)
      .ready(moveBanner)
      .ajaxComplete(moveBanner);
  })();


  /////////////////////////////////////////////////////////////////////////////// CHECKBOXES
  (function() {
    //------Add class to selected options on questions and covers on all pages
    function add_class_to_selected_option_on_question_and_quote_pages_for_checkboxes() {
      $(&quot; , &quot;'&quot; , &quot;label input[type=&quot;checkbox&quot;]&quot; , &quot;'&quot; , &quot;).each(function(){
        if ($(this).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
          $(this).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;ticked&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;unticked&quot; , &quot;'&quot; , &quot;);
        } else {
          $(this).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;unticked&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;ticked&quot; , &quot;'&quot; , &quot;);
        }
      });
    }

    //-----When clicked, on question pages
    $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click change&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;label input[type=&quot;checkbox&quot;]&quot; , &quot;'&quot; , &quot;, function(){
      if ($(this).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
        $(this).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;ticked&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;unticked&quot; , &quot;'&quot; , &quot;);
      } else {
        $(this).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;unticked&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;ticked&quot; , &quot;'&quot; , &quot;);
      }
    } );

    //-----Add class to selected options on questions and covers on all pages
    function add_class_to_disabled_option_on_question_and_quote_pages_for_checkboxes() {
      $(&quot; , &quot;'&quot; , &quot;label input[type=&quot;checkbox&quot;][disabled=&quot;disabled&quot;]&quot; , &quot;'&quot; , &quot;).each(function(){
        $(this).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
      });
    }

    //-----Run----------
    $(document).ready(function () {
      add_class_to_selected_option_on_question_and_quote_pages_for_checkboxes();
      add_class_to_disabled_option_on_question_and_quote_pages_for_checkboxes();
    });

    $(document).ajaxSuccess(function() {
      add_class_to_selected_option_on_question_and_quote_pages_for_checkboxes();
      add_class_to_disabled_option_on_question_and_quote_pages_for_checkboxes();
    });
  })();


  /**
   * Customer Portal.
   */
  (function() {
    setTimeout(function() {
      // open tab if there is hash value on url
      if (document.location.hash.length > 0) {
        $(&quot; , &quot;'&quot; , &quot;.nav-item a[href=&quot;&quot; , &quot;'&quot; , &quot;+document.location.hash+&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;).tab(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
      }

      // change navigate history when user switch tabs.
      $(&quot; , &quot;'&quot; , &quot;.nav-item a&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;shown.bs.tab&quot; , &quot;'&quot; , &quot;, function(e) {
        if (history.pushState) {
          window.history.pushState(null, null, e.target.hash);
        }
      });
    }, 0);

    if (Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;customerhomepage&quot; , &quot;'&quot; , &quot;) return;

    $(&quot; , &quot;'&quot; , &quot;.accordion-doc [data-toggle=&quot;collapse&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(event) {
      var collapse = $(event.target).next().collapse(&quot; , &quot;'&quot; , &quot;toggle&quot; , &quot;'&quot; , &quot;);
      $(event.target).attr(&quot; , &quot;'&quot; , &quot;aria-expanded&quot; , &quot;'&quot; , &quot;, collapse.attr(&quot; , &quot;'&quot; , &quot;aria-expanded&quot; , &quot;'&quot; , &quot;));
    });
  })();


  /**
   * Customer Portal Edit Details Page.
   *
   * Insert content to Edit Customer Details page
   */
  (function() {
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customerdetailschangepassword&quot; , &quot;'&quot; , &quot;){
      $(&quot; , &quot;'&quot; , &quot;.page-customerdetailschangepassword .instanda-main-content&quot; , &quot;'&quot; , &quot;).prepend(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;container&quot;>&lt;h3>Modification de votre mot de passe&lt;/h3>&lt;p style=&quot;margin-bottom: 40px;&quot;>Merci de renseigner ci-dessous votre nouveau mot de passe.&lt;/p>&lt;/div>&quot; , &quot;'&quot; , &quot;);
    }
  })();


  /**
   * Customer Portal Password Reset Sent.
   *
   * Add &quot;alert-success&quot; class to password reset succeeded message
   */
  (function() {
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customersendpasswordresetlink&quot; , &quot;'&quot; , &quot;){
      $(&quot; , &quot;'&quot; , &quot;#instanda-cp-reset-link-form .section_content&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;alert alert-success&quot; , &quot;'&quot; , &quot;);
    }
  })();


  /////////////////////////////////////////////////////////////////////////////// Translation
  (function() {
    // Post quote questions page 1.
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;postquotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber === 1) {
      $(&quot; , &quot;'&quot; , &quot;button[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;Souscrire en ligne&quot; , &quot;'&quot; , &quot;);
    }

    // Customer Portal Login Page
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customerlogin&quot; , &quot;'&quot; , &quot;) {
      $(&quot; , &quot;'&quot; , &quot;button[type=&quot;submit&quot;]&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Me connecter&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#Password&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Votre mot de passe&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;input[placeholder=&quot;email address&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Votre email&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#instanda-cp-forgot-link&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Je ne retrouve plus mon mot de passe&quot; , &quot;'&quot; , &quot;);
    }

    // Customer Portal Forget Password and Send Password Reset Pages
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customersendpasswordresetlink&quot; , &quot;'&quot; , &quot;) {
      $(&quot; , &quot;'&quot; , &quot;#instanda-cp-user-name&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Identifiant (email)&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;button[type=&quot;submit&quot;]&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Recevoir mon mot de passe par email&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#instanda-cp-reset-link-form a&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;Renvoyer&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;data-loading-text&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Renvoyer&quot; , &quot;'&quot; , &quot;);
    }

    // Customer Portal Input and Confirm New Password Page
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customerforgotpassword&quot; , &quot;'&quot; , &quot;) {
      $(&quot; , &quot;'&quot; , &quot;#NewPassword&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Votre nouveau mot de passe&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#ConfirmPassword&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Confirmez votre nouveau mot de passe&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#instanda-cp-charge-password&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Changer mon mot de passe&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;data-loading-text&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Changer mon mot de passe&quot; , &quot;'&quot; , &quot;);
    }

    // Customer Portal Your Account Details Page
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customerdetails&quot; , &quot;'&quot; , &quot;) {
      $(&quot; , &quot;'&quot; , &quot;label.control-label:contains(&quot;Username&quot;)&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Identifiant (email)&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;label.control-label:contains(&quot;Password&quot;)&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Mot de passe&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#password-fake-label + a&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;Modifier&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;.instanda-update-customer-details-button&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Mettre à jour&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#savemsg strong&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;Vos coordonnées ont été mises à jour&quot; , &quot;'&quot; , &quot;);
    }

    // Customer Portal Change Password Page (enter from Your Account Details Page)
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customerdetailschangepassword&quot; , &quot;'&quot; , &quot;) {
      $(&quot; , &quot;'&quot; , &quot;label.control-label:contains(&quot;Old&quot;)&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Votre mot de passe&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;label.control-label:contains(&quot;Confirm&quot;)&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Confirmez votre nouveau mot de passe&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;label.control-label:contains(&quot;New&quot;)&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Votre nouveau mot de passe&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#instanda-save-password-button&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Mettre à jour&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;data-loading-text&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;enregistrer&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;form a:contains(&quot;Cancel&quot;)&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;Annuler&quot; , &quot;'&quot; , &quot;);
    }

    // Registration
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;existingpolicyholdersendregistrationlink&quot; , &quot;'&quot; , &quot; || Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;publicuatuserlogin&quot; , &quot;'&quot; , &quot; || Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customerregister&quot; , &quot;'&quot; , &quot;) {
      $(&quot; , &quot;'&quot; , &quot;#instanda-cp-user-name&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Identifiant (email)&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;button[type=&quot;submit&quot;]&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Continuer&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;data-loading-text&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Continuer&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;label.control-label:contains(&quot;E-mail address&quot;)&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Votre email&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;label.control-label:contains(&quot;Password&quot;)&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Votre mot de passe&quot; , &quot;'&quot; , &quot;);
    }

    // Registration error message contains dynamic data, email address
    $(&quot; , &quot;'&quot; , &quot;.validation-summary-errors li&quot; , &quot;'&quot; , &quot;).each(function(index, elem) {
      const text = $(elem).text();
      const regex = /E-mail address &quot; , &quot;'&quot; , &quot;(.*)&quot; , &quot;'&quot; , &quot; has already been registered./;
      const matches = text.match(regex);

      if (matches) {
        $(elem).text(&quot; , &quot;'&quot; , &quot;Un compte existe déjà pour cette adresse e-mail. Merci de cliquer sur \&quot; , &quot;'&quot; , &quot;Me connecter\&quot; , &quot;'&quot; , &quot; pour accéder à votre Espace client.&quot; , &quot;'&quot; , &quot;);
      }
    });

    const translations = {
      &quot; , &quot;'&quot; , &quot;Log Out&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Déconnexion&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Log in&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Me connecter&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Your Account&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Vos coordonnées&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Continue&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Étape suivante&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Email address is required&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Adresse e-mail est nécessaire&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Please use a valid email address.&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Veuillez utiliser une adresse mail valide&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;The Password field is required.&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Le champ Mot de passe est obligatoire&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Password is required&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Mot de passe requis&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;New password is required&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Un nouveau mot de passe est requis&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Confirm password is required&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Confirmer le mot de passe est requis&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;City&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Ville&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Password must be at least 8 characters.&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Le mot de passe doit contenir au moins 8 caractères&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Password must contain at least 1 numbers.&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Le mot de passe doit contenir au moins 1 chiffre&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Passwords must include at least 1 special characters (!#@ etc)&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Le mot de passe doit contenir au moins 1 caractère spécial (!#@ etc)&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Invalid password&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Mot de passe incorrect&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Passwords do not match&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Les mots de passe ne correspondent pas&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Password must be between 6 and 99 characters&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Le mot de passe doit comporter entre 6 et 99 caractères&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;User is already registered&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Un compte lié à cette adresse email existe déjà&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;If that email address is in our database, we will send you an email to reset your password.&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Les instructions pour changer / recouvrer votre mot de passe vous ont été envoyés par email.&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Policy start date cannot be in the past&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;La date d\&quot; , &quot;'&quot; , &quot;effet du contrat ne peut pas être dans le passé. Pour finaliser votre contrat, merci de choisir une nouvelle date d\&quot; , &quot;'&quot; , &quot;effet. (+ indiquer qu\&quot; , &quot;'&quot; , &quot;il faut cliquer sur \&quot; , &quot;'&quot; , &quot;Précédent\&quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Authentication failed.&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Échec de l\&quot; , &quot;'&quot; , &quot;authentification&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;You are logged in with a different email address, you need to logout first to proceed with this quote&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Vous êtes connecté avec une adresse e-mail différente, vous devez d\’abord vous déconnecter pour continuer avec ce devis&quot; , &quot;'&quot; , &quot;,
&quot; , &quot;'&quot; , &quot;Logout and return to quote&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Déconnecter pour continuer avec ce devis&quot; , &quot;'&quot; , &quot;,
    }

    // @see https://stackoverflow.com/a/42041097/1146907
    const innerBody = document.getElementById(&quot; , &quot;'&quot; , &quot;innerBody&quot; , &quot;'&quot; , &quot;);
    const walker = document.createTreeWalker(innerBody, NodeFilter.SHOW_TEXT, null, false);
    while (walker.nextNode()) {
      for (const source in translations) {
        if (walker.currentNode.nodeValue.trim() == source) {
          walker.currentNode.nodeValue = translations[source];
        }
      }
    }

    const messageFields = document.querySelectorAll(&quot; , &quot;'&quot; , &quot;[data-valmsg-for]&quot; , &quot;'&quot; , &quot;);
    const callback = function(mutationList, observer) {
      for (let mutation of mutationList) {
        for (const source in translations) {
          if (mutation.addedNodes &amp;&amp; mutation.addedNodes.length > 0 &amp;&amp; mutation.addedNodes[0].innerText.trim() == source) {
            mutation.addedNodes[0].innerText = translations[source];
          }
        }
      }
    }
    const observer = new MutationObserver(callback);
    const config = { childList: true };
    messageFields.forEach(function(field) {
      observer.observe(field, config);
    });

  })();


// HXF-3 - Postcode look up
const lookUpPostcode = function() {
  var _PostcodeFields = [];
  _PostcodeFields.push({
    postcode: &quot;input#Postcode118934&quot;,
    city: &quot;input#City&quot;,
    container: &quot;.form-group&quot;,
    wrapper: &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;instanda-question-item form-group container postcode-look-up&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;
  }); // Page 3
  _PostcodeFields.push({
    postcode: &quot;input#Postcode_FormB_NUM&quot;,
    city: &quot;input#City_FormB&quot;,
    container: &quot;.questionItem&quot;,
    wrapper: &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;questionItem instanda-question-item form-group container postcode-look-up&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;
  }); // Page 4
  _PostcodeFields.push({
    postcode: &quot;input#CompanyPostcode_TXT&quot;,
    city: &quot;input#CompanyCity_TXT&quot;,
    container: &quot;.questionItem&quot;,
    wrapper: &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;questionItem instanda-question-item form-group container postcode-look-up&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;
  }); // Page 5

  $.each(_PostcodeFields, function(key, value){
    $(value.city).prop(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;, true).trigger(&quot;mdc:rebuild&quot;);

    window.setTimeout(function () {
      if ($(value.postcode).length > 0) {
        $(value.postcode).on(&quot;change&quot;, function () {
          if (typeof $(value.postcode).val() !== &quot;undefined&quot; &amp;&amp; $(value.postcode).val().trim() !== &quot;&quot;) {
            $.get(&quot;https://geo.api.gouv.fr/communes&quot;, { codePostal: $(value.postcode).val().replace(/ /g, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) }).done(
              function (data) {
                if ($(&quot;select.gc-fr-postale-code&quot;).length > 0) {
                  $(&quot;select.gc-fr-postale-code&quot;).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-item&quot; , &quot;'&quot; , &quot;).remove();
                }
                $(value.city+&quot; , &quot;'&quot; , &quot;--mdc&quot; , &quot;'&quot; , &quot;).focus();
                $(value.city).val(&quot;&quot;).trigger(&quot;change&quot;);
                if (data.length > 0) {
                  var _postal = data;
                  if (typeof _postal !== &quot;undefined&quot; &amp;&amp; _postal !== null) {
                    if(_postal.length === 1){
                      $(value.city).val(_postal[0].nom).trigger(&quot;change&quot;);
                    } else {
                      var _ddl = $(&quot;&lt;select>&quot;)
                        .addClass(&quot;gc-fr-postale-code&quot;)
                        .on(&quot;change&quot;, function(){
                          $(value.city).val($(this).val()).trigger(&quot;mdc:rebuild&quot;);
                        });
                      $.each(_postal, function(key, value){
                        $(_ddl).append(
                          $(&quot;&lt;option/>&quot;)
                          .attr({ value: value.nom })
                          .text(value.nom)
                        );
                      });
                      $(value.city).val(_ddl.val()).trigger(&quot;mdc:rebuild&quot;);
                      var _ddlContainer = $(value.wrapper).append(_ddl);
                      $(value.city+&quot; , &quot;'&quot; , &quot;--mdc&quot; , &quot;'&quot; , &quot;).closest(value.container).before(_ddlContainer);
                    }
                  }
                } else {
                  //$(value.city).focus();
                  $(value.city).val(&quot;&quot;); //.trigger(&quot;change&quot;);
                }
              }
            );
          }
        });
      }
    });
  },2000);
}

$(document).ready(lookUpPostcode);



  /* Quote Display */
  (function () {
    // send email and continue in one click
    if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot; &amp;&amp; (Instanda.Variables.PageNumber == 1 || Instanda.Variables.PageNumber == 5 )) {
      const fakeBtn = $(&quot; , &quot;'&quot; , &quot;.sidebar-cta .btn-fake&quot; , &quot;'&quot; , &quot;);
      const nextBtn = $(&quot; , &quot;'&quot; , &quot;.sidebar-cta .instanda_nav_link&quot; , &quot;'&quot; , &quot;).hide();
      const nextBtnBelow = $(&quot; , &quot;'&quot; , &quot;#continueButton&quot; , &quot;'&quot; , &quot;).hide();
      const fakeBtnBelow = $(&quot; , &quot;'&quot; , &quot;&lt;button type=&quot;submit&quot; name=&quot;continueButton&quot; class=&quot;instanda-button instanda-quote-continue-button btn btn-primary&quot;>Recevoir mon devis par email &lt;i class=&quot;fa fa-caret-right&quot;>&lt;/i>&lt;/button>&quot; , &quot;'&quot; , &quot;).insertBefore(nextBtnBelow); 
      const emailLink = $(&quot; , &quot;'&quot; , &quot;#emailQuoteLinkLink&quot; , &quot;'&quot; , &quot;).hide();

      fakeBtnBelow.on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function () {
        emailLink.trigger(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;);
        /* commented by RitaHiscoxFrance 13/04/2022 INST #20373 - begin {
        setTimeout(function () {
          nextBtnBelow.trigger(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;);
        }, 200);
        \* end } - commented by RitaHiscoxFrance 13/04/2022 INST #20373 */
      });

      fakeBtn.on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function () {
        emailLink.trigger(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;);
        setTimeout(function () {
          nextBtn.trigger(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;);
        }, 200);
      });
    }
  })();



  // swap address fields order
  (function() {
    const addressContainer = $(&quot; , &quot;'&quot; , &quot;#question118934&quot; , &quot;'&quot; , &quot;);
    if (addressContainer.length > 0) {
      const postCode = $(&quot; , &quot;'&quot; , &quot;.instanda-address-postcode&quot; , &quot;'&quot; , &quot;, addressContainer).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-item&quot; , &quot;'&quot; , &quot;);
      const city = $(&quot; , &quot;'&quot; , &quot;.instanda-address-city&quot; , &quot;'&quot; , &quot;, addressContainer).closest(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;);
      if (postCode.length > 0 &amp;&amp; city.length > 0) {
        postCode.insertBefore(city);
      }
    }
  })();


  /**
   * Post-Quote Questions (Page 1).
   */
  (function() {
    // set download document URL
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;postquotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
      const matches = document.location.search.match(/[?|&amp;]quoteRef=(.*?)[$|&amp;]/);
      const quoteRef = matches[1];

      if (Instanda.Variables.InsuranceType_CHOICE === &quot; , &quot;'&quot; , &quot;Votre activité professionnelle&quot; , &quot;'&quot; , &quot;) {
        const docLink = &quot; , &quot;'&quot; , &quot;/Public/DownloadStoredPdfBuilder?packageId=&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;12462&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&amp;pdfBuilderId=&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;305624&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&amp;quoteRef=&quot; , &quot;'&quot; , &quot; + quoteRef;
        const link = $(&quot; , &quot;'&quot; , &quot;.download-link&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, docLink);
      }

      if (Instanda.Variables.InsuranceType_CHOICE === &quot; , &quot;'&quot; , &quot;Vos locaux&quot; , &quot;'&quot; , &quot;) {
        const docLink = &quot; , &quot;'&quot; , &quot;/Public/DownloadStoredPdfBuilder?packageId=&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;12462&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&amp;pdfBuilderId=&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;305637&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&amp;quoteRef=&quot; , &quot;'&quot; , &quot; + quoteRef;
        const link = $(&quot; , &quot;'&quot; , &quot;.download-link&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, docLink);
      }

      if (Instanda.Variables.InsuranceType_CHOICE === &quot; , &quot;'&quot; , &quot;Votre activité &amp; vos locaux&quot; , &quot;'&quot; , &quot;) {
        const docLink = &quot; , &quot;'&quot; , &quot;/Public/DownloadStoredPdfBuilder?packageId=&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;12462&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&amp;pdfBuilderId=&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;305652&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&amp;quoteRef=&quot; , &quot;'&quot; , &quot; + quoteRef;
        const link = $(&quot; , &quot;'&quot; , &quot;.download-link&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, docLink);
      }
    }
  })();


  /**
   * Post-Quote Questions (Page 2).
   */
  (function() {
    // set download document URL
    if ((Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;postquotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;) || (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;postquotequestionscontinue&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;)) {
      const modifyLink = function() {
        const $modifyLink = $(&quot; , &quot;'&quot; , &quot;.policy-start-date [data-placeholder=&quot;~publicPreQuoteLink&quot;]&quot; , &quot;'&quot; , &quot;);
        const href = $modifyLink.prop(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;);
        const url = new URL(href);
        url.searchParams.set(&quot; , &quot;'&quot; , &quot;pageNumber&quot; , &quot;'&quot; , &quot;, 3);
        url.searchParams.set(&quot; , &quot;'&quot; , &quot;goBackwards&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;);
        $modifyLink.prop(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, url.toString());
      }
      $(document).ready(modifyLink);

      const fillDummyAddress = function() {
        setTimeout(function () {
            $(&quot; , &quot;'&quot; , &quot;#InsuredAddressLine1_TXT&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#InsuredAddressLine2_TXT&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;Street&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#InsuredAddressCity_TXT&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;City&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;input[name=&quot;InsuredPostcode_TXT&quot;]&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;12345&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
        }, 0);
      }
      $(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, fillDummyAddress);
    }
  })();


  /* Phone widget */
  (function() {
    $(&quot; , &quot;'&quot; , &quot;.tel-icon-wannaspeak&quot; , &quot;'&quot; , &quot;).click(function() {
      wsCall();
    });
  })();


// START
// HXF-8: [PI Public] Real time class toggling on valid fields
// GC 01/10/2020
$(document).ready(function(){
    // Creates and returns a unique id
    function uuidv4() {
      return &quot; , &quot;'&quot; , &quot;xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx&quot; , &quot;'&quot; , &quot;.replace(/[xy]/g, function(c) {
        var r = Math.random() * 16 | 0, v = c == &quot; , &quot;'&quot; , &quot;x&quot; , &quot;'&quot; , &quot; ? r : (r &amp; 0x3 | 0x8);
        return v.toString(16);
      });
    };
    // Configure our fields
    var _fieldsToValidate = [];
    _fieldsToValidate.push({ &quot;ValidOn&quot;: &quot;range&quot;, &quot;Parent&quot;: &quot;div#question119075&quot;, &quot;Selector&quot;: &quot;input#IBAN_TXT&quot;, &quot;Type&quot;: &quot;Text&quot;, &quot;Mdc&quot;: true, &quot;On&quot;: &quot;change input&quot;, &quot;Min&quot;: 27, &quot;Max&quot;: 27, &quot;Css&quot;: &quot;realtime-is-valid&quot; });
    _fieldsToValidate.push({ &quot;ValidOn&quot;: &quot;other&quot;, &quot;Parent&quot;: &quot;div#question119076&quot;, &quot;Selector&quot;: &quot;input#BIC_TXT&quot;, &quot;Type&quot;: &quot;Text&quot;, &quot;Mdc&quot;: true, &quot;On&quot;: &quot;change input&quot;, &quot;Min&quot;: 8, &quot;Max&quot;: 11, &quot;Css&quot;: &quot;realtime-is-valid&quot; });
    _fieldsToValidate.push({ &quot;ValidOn&quot;: &quot;range&quot;, &quot;Parent&quot;: &quot;div#question119077&quot;, &quot;Selector&quot;: &quot;input#Domiciliation_TXT&quot;, &quot;Type&quot;: &quot;Text&quot;, &quot;Mdc&quot;: true, &quot;On&quot;: &quot;change input&quot;, &quot;Min&quot;: 1, &quot;Max&quot;: 999, &quot;Css&quot;: &quot;realtime-is-valid&quot; });
    _fieldsToValidate.push({ &quot;ValidOn&quot;: &quot;range&quot;, &quot;Parent&quot;: &quot;div#question119078&quot;, &quot;Selector&quot;: &quot;input#AccountOwner_TXT&quot;, &quot;Type&quot;: &quot;Text&quot;, &quot;Mdc&quot;: true, &quot;On&quot;: &quot;change input&quot;, &quot;Min&quot;: 1, &quot;Max&quot;: 999, &quot;Css&quot;: &quot;realtime-is-valid&quot; });
    // Initialize our fields
    // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
    // GC 08/10/2020
    // Switch statement added to increase the scope of the validation.
    // BIC is now not a range between 8 and 11 but is either 8 OR 11.
    $.each(_fieldsToValidate, function(key, value){
        if($(value.Parent).length > 0){
            var _guid = uuidv4();
            value.Guid = _guid; // for use when validating
            $(&quot;&lt;span data-span-guid=&quot; , &quot;'&quot; , &quot;&quot; + _guid + &quot;&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;realtime-valid&quot; , &quot;'&quot; , &quot; />&quot;).prependTo(value.Parent);
            // Set tab-index for focus capture
            $(value.Parent).attr({ &quot;data-realtime&quot;: JSON.stringify(value) }).attr({ &quot;tab-index&quot;: key });
            $(value.Parent).on(&quot;click&quot;, function(){
                // When we are focused in the question div bind the behaviour...
                // ...mdc compontent strips it at every focus out...
                var _data = $(this).attr(&quot;data-realtime&quot;); // data attached to question div...
                _data = JSON.parse(_data);
                $(_data.Selector, _data.Parent).data(_data).on(_data.On, function(el){
                    var _d = $(el.target).data(); // data attached to input...
                    var _thisVal = $(_d.Selector, _d.Parent).val();
                    // Check the value matches the input and hide/show css class
                    console.log(&quot;_d ================> &quot;, _d);
                    if(typeof _d !== &quot;undefined&quot; &amp;&amp; typeof _d.ValidOn !== &quot;undefined&quot;){
                        switch(_d.ValidOn.toLowerCase()){
                            case &quot;range&quot;:
                                if (typeof _thisVal !== &quot;undefined&quot; &amp;&amp; _thisVal !== null &amp;&amp; _thisVal.length >= _d.Min &amp;&amp; _thisVal.length &lt;= _d.Max) {
                                    $(_d.Parent).addClass(&quot;realtime-is-valid&quot;);
                                    $(&quot;span[data-span-guid=&quot; , &quot;'&quot; , &quot;&quot; + _d.Guid + &quot;&quot; , &quot;'&quot; , &quot;]&quot;).addClass(_d.Css);
                                } else {
                                    $(_d.Parent).removeClass(&quot;realtime-is-valid&quot;);
                                    $(&quot;span[data-span-guid=&quot; , &quot;'&quot; , &quot;&quot; + _d.Guid + &quot;&quot; , &quot;'&quot; , &quot;]&quot;).removeClass(_d.Css);
                                }
                                break;
                            case &quot;other&quot;:
                                if (typeof _thisVal !== &quot;undefined&quot; &amp;&amp; _thisVal !== null &amp;&amp; (_thisVal.length === _d.Min || _thisVal.length === _d.Max)) {
                                    $(_d.Parent).addClass(&quot;realtime-is-valid&quot;);
                                    $(&quot;span[data-span-guid=&quot; , &quot;'&quot; , &quot;&quot; + _d.Guid + &quot;&quot; , &quot;'&quot; , &quot;]&quot;).addClass(_d.Css);
                                } else {
                                    $(_d.Parent).removeClass(&quot;realtime-is-valid&quot;);
                                    $(&quot;span[data-span-guid=&quot; , &quot;'&quot; , &quot;&quot; + _d.Guid + &quot;&quot; , &quot;'&quot; , &quot;]&quot;).removeClass(_d.Css);
                                }
                                break;
                        }
                    }
                });
            });
            // GC added to trigger the code to check for previous values and autocompletes...
            setTimeout(function(){
                $(value.Parent).trigger(&quot;click&quot;);
                $(value.Selector + (value.Mdc ? &quot;--mdc&quot; : &quot;&quot;), value.Parent).keyup();
            }, 500);
        }
    });
    // GC 08/10/2020
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
});
// END
// HXF-8: [PI Public] Real time class toggling on valid fields
// GC 01/10/2020


  //////////////////////////////////////////////////////////////////////////////// LOADING ANIMATION

  // Wait for window load, custom-made loading effect
  $(document).ready(function() {
    // Animate loader off screen
    $(&quot;.custom-loading-wrapper&quot;).fadeOut(&quot;slow&quot;);
  });

  //wait for page submit, custom-made loading effect.
  document.addEventListener(&quot; , &quot;'&quot; , &quot;submit&quot; , &quot;'&quot; , &quot;, function() {
    ShowWaitingAnimation();
  });


  //////////////////////////////////////////////////////////////////////////////// Honeypot
  $(&quot; , &quot;'&quot; , &quot;#HoneypotHiddenQuestion&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;tabindex&quot; , &quot;'&quot; , &quot;, -1);



//Create the button Valider - Promocode ( Quote adjustment questions )

function insertPromocodeButton() {
	if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot; &amp;&amp; (Instanda.Variables.PageNumber == 1 || Instanda.Variables.PageNumber == 5 )) {
		updateClone = $(&quot;.instanda-quote-update-button&quot;).clone();
		updateClone.attr(&quot;name&quot;, &quot;promocodeBtn&quot;);  
		updateClone.insertAfter(&quot;.promocode .questionItem &quot;);
		updateClone.attr(&quot;class&quot;, &quot;promocode&quot;);  
		var promo_info = $(&quot;.promocode .instanda-question-help-col&quot;);
		promo_info.hide();
		if ($(&quot;.alert.alert-danger.alert-dismissible.show&quot;).length > 0 &amp;&amp; Instanda.Variables.DeclineRule_Promo_PublicSite == &quot;NotDeclineRule&quot;) {
			updateClone.attr(&quot;class&quot;, &quot;promocode error-message&quot;);  
		}
		if ( Instanda.Variables.IsRenewal &amp;&amp; Instanda.Variables.Renewal_Promo == &quot;No&quot;) {
			 Instanda.Variables.PromocodePublic = &quot;&quot;;
		}

	}
}
insertPromocodeButton();


            
            
            
            
                if (!isTouchDevice) {
                    document.write(&quot; , &quot;'&quot; , &quot;\x3Cscript type=&quot;text/javascript&quot; src=&quot;/Theme/js/plugins/bootstrap-miscellaneous/bootstrap-datetimepicker.min.js&quot;>\x3C/script>&quot; , &quot;'&quot; , &quot;);
                }

                // Clear forms fields when then browser back button is used
                $(window).on(&quot;load&quot;, null, function () {

                    var $forms = $(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;);

                    if ($forms.length !== 0) {

                        $forms.get(0).reset();
                    }
                });
            
            
            
                const clientOriginalShortDatePatternForDatePicker = &quot; , &quot;'&quot; , &quot;dd/mm/yy&quot; , &quot;'&quot; , &quot;;
                const clientOriginalShortDatePattern = &quot; , &quot;'&quot; , &quot;DD/MM/YYYY&quot; , &quot;'&quot; , &quot;;
                datepickerLogic(clientOriginalShortDatePatternForDatePicker, clientOriginalShortDatePattern, isTouchDevice, isResponsiveTheme)
            


    
        
            
                ×
                JavaScript Error
            
            
                ... error text
            
            
                Close
            
        
    

        





            
                

                Confidentialité &amp; Protection des données Conditions d’utilisation et Mentions Légales Politique de cookies

© 2021 Hiscox SA. All rights reserved by Hiscox SA.

                    Site built and hosted using
                    Instanda

    v1.84.0.5

                    insurance software.
                
            


        


    
        function PopUpReadOnlyToaster() {
            toastr.options = {
                &quot;closeButton&quot;: true,
			    &quot;debug&quot;: false,
			    &quot;newestOnTop&quot;: false,
			    &quot;progressBar&quot;: false,
			    &quot;positionClass&quot;: &quot;toast-bottom-full-width&quot;,
			    &quot;preventDuplicates&quot;: true,
			    &quot;onclick&quot;: null,
			    &quot;showDuration&quot;: &quot;300&quot;,
			    &quot;hideDuration&quot;: &quot;1000&quot;,
			    &quot;timeOut&quot;: &quot;5000&quot;,
			    &quot;extendedTimeOut&quot;: &quot;1000&quot;,
			    &quot;showEasing&quot;: &quot;swing&quot;,
			    &quot;hideEasing&quot;: &quot;linear&quot;,
			    &quot;showMethod&quot;: &quot;fadeIn&quot;,
			    &quot;hideMethod&quot;: &quot;fadeOut&quot;,
                &quot;escapeHtml&quot; : true
            }



            toastr[&quot;info&quot;](&quot; , &quot;'&quot; , &quot;This quote is displayed as readonly&quot; , &quot;'&quot; , &quot;);
        }

            
            &quot;use strict&quot;;
			$(function () {
                var common = new Instanda.Common();
                var keeplLockOnPolicyInterval = window.setInterval(function ()

                    {
                    if (!common.KeepLockOnPolicy(&quot; , &quot;'&quot; , &quot;Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&quot; , &quot;'&quot; , &quot;))
                        clearInterval(keeplLockOnPolicyInterval);
                }, 60000);
            });

            


    

        
        
        
        
    






function OptanonWrapper(){window.dataLayer.push({event:&quot;OneTrustGroupsUpdated&quot;})};

(function(){var a=document.createElement(&quot;script&quot;);a.id=&quot;ws-diddyn-lib&quot;;a.type=&quot;text/javascript&quot;;a.async=!0;a.defer=!0;a.src=&quot;//cdn.wannaspeak.com/lib/did.js&quot;;var b=document.getElementsByTagName(&quot;script&quot;)[0];b.parentNode.insertBefore(a,b)})();
(function(c,d,f,g,e){c[e]=c[e]||[];var h=function(){var b={ti:&quot; 4010679&quot;};b.q=c[e];c[e]=new UET(b);c[e].push(&quot;pageLoad&quot;)};var a=d.createElement(f);a.src=g;a.async=1;a.onload=a.onreadystatechange=function(){var b=this.readyState;b&amp;&amp;&quot;loaded&quot;!==b&amp;&amp;&quot;complete&quot;!==b||(h(),a.onload=a.onreadystatechange=null)};d=d.getElementsByTagName(f)[0];d.parentNode.insertBefore(a,d)})(window,document,&quot;script&quot;,&quot;//bat.bing.com/bat.js&quot;,&quot;uetq&quot;);

  
  

var axel=Math.random()+&quot;&quot;,a=1E13*axel;document.write(&quot;\x3ciframe src\x3dhttps://6415832.fls.doubleclick.net/activityi;src\x3d6415832;type\x3dhpt5r0;cat\x3dhisco0;dc_lat\x3d;dc_rdid\x3d;tag_for_child_directed_treatment\x3d;tfua\x3d;npa\x3d;gdpr\x3d${GDPR};gdpr_consent\x3d${GDPR_CONSENT_755};ord\x3d1;num\x3d&quot;+a+&quot; , &quot;'&quot; , &quot;? width\x3d&quot;1&quot; height\x3d&quot;1&quot; frameborder\x3d&quot;0&quot; style\x3d&quot;display:none&quot;\x3e\x3c/iframe\x3e&quot; , &quot;'&quot; , &quot;);


window._fs_host=&quot;eu1.fullstory.com&quot;;window._fs_script=&quot;edge.eu1.fullstory.com/s/fs.js&quot;;window._fs_org=&quot;o-18D5-eu1&quot;;window._fs_namespace=&quot;FS&quot;;
(function(e,k,l,m,h,f,a,d){l in e?e.console&amp;&amp;e.console.log&amp;&amp;e.console.log(&quot; , &quot;'&quot; , &quot;FullStory namespace conflict. Please set window[&quot;_fs_namespace&quot;].&quot; , &quot;'&quot; , &quot;):(a=e[l]=function(c,b,g){a.q?a.q.push([c,b,g]):a._api(c,b,g)},a.q=[],f=k.createElement(m),f.async=1,f.crossOrigin=&quot;anonymous&quot;,f.src=&quot;https://&quot;+_fs_script,d=k.getElementsByTagName(m)[0],d.parentNode.insertBefore(f,d),a.identify=function(c,b,g){a(h,{uid:c},g);b&amp;&amp;a(h,b,g)},a.setUserVars=function(c,b){a(h,c,b)},a.event=function(c,b,g){a(&quot;event&quot;,{n:c,p:b},g)},a.anonymize=
function(){a.identify(!1)},a.shutdown=function(){a(&quot;rec&quot;,!1)},a.restart=function(){a(&quot;rec&quot;,!0)},a.log=function(c,b){a(&quot;log&quot;,[c,b])},a.consent=function(c){a(&quot;consent&quot;,!arguments.length||c)},a.identifyAccount=function(c,b){f=&quot;account&quot;;b=b||{};b.acctId=c;a(f,b)},a.clearUserCookie=function(){},a.setVars=function(c,b){a(&quot;setVars&quot;,[c,b])},a._w={},d=&quot;XMLHttpRequest&quot;,a._w[d]=e[d],d=&quot;fetch&quot;,a._w[d]=e[d],e[d]&amp;&amp;(e[d]=function(){return a._w[d].apply(this,arguments)}),a._v=&quot;1.3.0&quot;)})(window,document,window._fs_namespace,
&quot;script&quot;,&quot;user&quot;);




  window.teads_e=window.teads_e||[];window.teads_adv_id=19472;
Centre de préférences de la confidentialitéLorsque vous consultez un site Web, des données peuvent être stockées dans votre navigateur ou récupérées à partir de celui-ci, généralement sous la forme de cookies. Ces informations peuvent porter sur vous, sur vos préférences ou sur votre appareil et sont principalement utilisées pour s&quot; , &quot;'&quot; , &quot;assurer que le site Web fonctionne correctement. Les informations ne permettent généralement pas de vous identifier directement, mais peuvent vous permettre de bénéficier d&quot; , &quot;'&quot; , &quot;une expérience Web personnalisée. Parce que nous respectons votre droit à la vie privée, nous vous donnons la possibilité de ne pas autoriser certains types de cookies. Cliquez sur les différentes catégories pour obtenir plus de détails sur chacune d&quot; , &quot;'&quot; , &quot;entre elles, et modifier les paramètres par défaut. Toutefois, si vous bloquez certains types de cookies, votre expérience de navigation et les services que nous sommes en mesure de vous offrir peuvent être impactés. Pour modifier vos choix et préférences à tout moment, rendez-vous sur notre lien ‘politique de cookies’ :  
            https://www.hiscox.fr/politique-de-cookiesTout autoriserGérer les préférences de consentementCookies strictement nécessairesToujours actifIl s’agit des cookies indispensables au fonctionnement d’un site web. Sont inclus, par exemple, les cookies qui permettent aux utilisateurs de se connecter à des espaces sécurisés du site web, d’utiliser un panier d’achat ou d’utiliser les services de facturation électronique. Cliquez ici pour une liste d’exemples de cookies strictement nécessaires.Cookies d’analyse et de performance  Cookies d’analyse et de performance Ces cookies nous permettent de déterminer le nombre de visites et les sources du trafic sur notre site web, afin d&quot; , &quot;'&quot; , &quot;en mesurer et d’en améliorer les performances. Ils nous aident également à identifier les pages les plus / moins visitées et à évaluer comment les visiteurs naviguent sur le site. Toutes les informations, collectées par ces cookies, sont agrégées et donc anonymisées. Si vous n&quot; , &quot;'&quot; , &quot;acceptez pas cette catégorie de cookies, nous ne pourrons pas savoir quand vous avez réalisé votre visite sur notre site web.Cookies de fonctionalité  Cookies de fonctionalité Ils sont utilisés pour reconnaître les utilisateurs lorsqu&quot; , &quot;'&quot; , &quot;ils reviennent sur un site web. Ils permettent la personnalisation du contenu, la reconnaissance des utilisateurs et mémorisent les préférences des utilisateurs (par exemple, leur choix de langue ou de région). Cliquez ici pour une liste d’exemples de cookies de fonctionnalités.Cookies pour une publicité ciblée  Cookies pour une publicité ciblée Ces cookies peuvent être activés sur notre site web par nos partenaires publicitaires. Ils peuvent être utilisés par ces entreprises pour établir des profils sur vos intérêts, et afin de vous proposer des publicités ciblées sur d’autres sites. Ils fonctionnement uniquement en identifiant votre navigateur et votre appareil. Si vous n&quot; , &quot;'&quot; , &quot;acceptez pas cette catégorie de cookies, des publicités moins ciblées sur vos intérêts vous seront proposées lors de votre navigation sur d&quot; , &quot;'&quot; , &quot;autres sites web.Cookies de tiers  Cookies de tiers Lorsque vous visitez notre site web, vous pouvez remarquer des cookies qui ne nous sont pas liés. Lorsque vous consultez une page proposant du contenu intégré, par exemple sur YouTube, des cookies peuvent vous être envoyés à partir de ces sites web. Nous ne contrôlons pas la configuration de ces cookies et nous vous conseillons de consulter les sites web des tiers pour plus d&quot; , &quot;'&quot; , &quot;informations sur ces cookies et sur la façon de les gérer.

Pour désactiver toutes les publicités Hiscox ciblées sur les sites web d&quot; , &quot;'&quot; , &quot;autres sociétés, veuillez consulter http://www.youronlinechoices.com/fr/ , où vous pourrez désactiver les réseaux de publicité Microsoft, Amazon, Google, AppNexus, Turn, Facebook et Twitter. Veuillez noter qu&quot; , &quot;'&quot; , &quot;en désactivant les réseaux de publicité listés, vous désactiverez les publicités ciblées pour Hiscox et toute autre entreprise utilisant ces réseaux.

Hiscox utilise des fournisseurs qui peuvent placer des cookies sur le site web d’Hiscox afin de fournir divers services.Back ButtonListe des cookies Search IconFilter IconClear checkbox label labelApply CancelConsent Leg.Interest checkbox label label checkbox label label checkbox label label Confirmer la sélection(function(){var a=document.createElement(&quot;script&quot;);a.id=&quot;ws-diddyn-lib&quot;;a.type=&quot;text/javascript&quot;;a.async=!0;a.defer=!0;a.src=&quot;//cdn.wannaspeak.com/lib/did.js&quot;;var b=document.getElementsByTagName(&quot;script&quot;)[0];b.parentNode.insertBefore(a,b)})();
  
  

var axel=Math.random()+&quot;&quot;,a=1E13*axel;document.write(&quot;\x3ciframe src\x3dhttps://6415832.fls.doubleclick.net/activityi;src\x3d6415832;type\x3dhpt5r0;cat\x3dhisco0;dc_lat\x3d;dc_rdid\x3d;tag_for_child_directed_treatment\x3d;tfua\x3d;npa\x3d;gdpr\x3d${GDPR};gdpr_consent\x3d${GDPR_CONSENT_755};ord\x3d1;num\x3d&quot;+a+&quot; , &quot;'&quot; , &quot;? width\x3d&quot;1&quot; height\x3d&quot;1&quot; frameborder\x3d&quot;0&quot; style\x3d&quot;display:none&quot;\x3e\x3c/iframe\x3e&quot; , &quot;'&quot; , &quot;);


window._fs_host=&quot;eu1.fullstory.com&quot;;window._fs_script=&quot;edge.eu1.fullstory.com/s/fs.js&quot;;window._fs_org=&quot;o-18D5-eu1&quot;;window._fs_namespace=&quot;FS&quot;;
(function(e,k,l,m,h,f,a,d){l in e?e.console&amp;&amp;e.console.log&amp;&amp;e.console.log(&quot; , &quot;'&quot; , &quot;FullStory namespace conflict. Please set window[&quot;_fs_namespace&quot;].&quot; , &quot;'&quot; , &quot;):(a=e[l]=function(c,b,g){a.q?a.q.push([c,b,g]):a._api(c,b,g)},a.q=[],f=k.createElement(m),f.async=1,f.crossOrigin=&quot;anonymous&quot;,f.src=&quot;https://&quot;+_fs_script,d=k.getElementsByTagName(m)[0],d.parentNode.insertBefore(f,d),a.identify=function(c,b,g){a(h,{uid:c},g);b&amp;&amp;a(h,b,g)},a.setUserVars=function(c,b){a(h,c,b)},a.event=function(c,b,g){a(&quot;event&quot;,{n:c,p:b},g)},a.anonymize=
function(){a.identify(!1)},a.shutdown=function(){a(&quot;rec&quot;,!1)},a.restart=function(){a(&quot;rec&quot;,!0)},a.log=function(c,b){a(&quot;log&quot;,[c,b])},a.consent=function(c){a(&quot;consent&quot;,!arguments.length||c)},a.identifyAccount=function(c,b){f=&quot;account&quot;;b=b||{};b.acctId=c;a(f,b)},a.clearUserCookie=function(){},a.setVars=function(c,b){a(&quot;setVars&quot;,[c,b])},a._w={},d=&quot;XMLHttpRequest&quot;,a._w[d]=e[d],d=&quot;fetch&quot;,a._w[d]=e[d],e[d]&amp;&amp;(e[d]=function(){return a._w[d].apply(this,arguments)}),a._v=&quot;1.3.0&quot;)})(window,document,window._fs_namespace,
&quot;script&quot;,&quot;user&quot;);
  window.teads_e=window.teads_e||[];window.teads_adv_id=19472;
id(&quot;LastName--mdc&quot;)&quot;) or . = concat(&quot;


&lt;iframe src=&quot;https://www.googletagmanager.com/ns.html?id=GTM-NZRGST&quot;
height=&quot;0&quot; width=&quot;0&quot; style=&quot;display:none;visibility:hidden&quot;>&lt;/iframe>

        
        
        
Vous avez une question ? 01 89 07 75 97Appel gratuit | Du lundi au vendredi 08h30 - 20h00 hors jours fériés

    
        
            
                
                    Toggle navigation
                    
                    
                    
                

                    
                        
                        
                    
            
Votre profilVotre besoin Votre devis Vous êtes assuré


            

Vous avez une question ? 01 89 07 75 97Appel gratuit
Du lundi au vendredi 08h30 - 20h00
hors jours fériés



01 89 07 75 97Appel gratuit du lundi au vendredi 08h30 - 19h00 hors jours fériés


                

                                
Obtenir un devis                                
                                
Me connecter                                

                
            
        
    



                    







﻿﻿Estimez gratuitement le coût de votre assurance professionnelle

Votre devis Responsabilité Civile Professionnelle

Votre contrat Responsabilité Civile Professionnelle



    .navbar-static-top .nav {
        padding: 15px 0;
    }

    @media (max-width: 767px) {
        .navbar-brand {
            padding: 0;
        }

            .navbar-brand img {
                margin-top: 5px;
                margin-left: 13px;
            }
    }





×





                    
                        Pourriez-vous remplir les informations suivantes ?
                    

                                
                    


 

                    


            

                
                


	



                
                








        var $questionForm = null,
            $quoteContinueButton = null,
            newLocation = &quot;&quot;,
            redirect = &quot;False&quot; == &quot;True&quot;;

        if (redirect) {
            newLocation = &quot;/Public/Quote?PackageId=12462&amp;amp;quoteRef=Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&amp;amp;quoteIsReadOnly=False&amp;amp;fromNavLink=False&quot;;
            window.open(newLocation, &quot;_self&quot;);
        }

        $(function () {
            &quot;use strict&quot;;

            $quoteContinueButton = $(&quot;#continueButton&quot;);

            $questionForm = $quoteContinueButton
                .parentsUntil(&quot;form&quot;)
                .parent();

            if ($quoteContinueButton.length !== 0 &amp;&amp; $questionForm.length !== 0) {

                $quoteContinueButton.on(&quot;click&quot;, null, function () {

                    $questionForm
                        .find(&quot;#goBackward&quot;)
                        .remove()
                        .end()
                        .submit();
                });
            }

            ShowHelpTextOnFocus(&quot;WhenClicked&quot;);

            $(document.body).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.form-control[data-val-regex-pattern], .form-control[data-val-regexci-pattern]&quot; , &quot;'&quot; , &quot;, function () {
                var txtVal = $(this).val().trim();
                $(this).val(txtVal);
            });
        });




		
			
            



            
            
                



            


                    
                        Le preneur d&quot; , &quot;'&quot; , &quot;assurance
                    

            



            



        
            Votre nom
        
    
        




        

            
                
                    





                    
                        


                            
                            Madame
                        
                    
                    
                        


                            
                            Monsieur
                        
                    
            
                
            




    

       var func_Titre = (function(){

           var showOptionalText = false;

            if (showOptionalText) {

                var Titre = &quot; , &quot;'&quot; , &quot;Titre&quot; , &quot;'&quot; , &quot;;
                var dropdownadditionalchoiceinfo118957 = &quot; , &quot;'&quot; , &quot;dropdownadditionalchoiceinfo118957&quot; , &quot;'&quot; , &quot;;

                var extraInfoSelector = &quot;#&quot; + &quot; , &quot;'&quot; , &quot;dropdownadditionalchoiceinfo118957&quot; , &quot;'&quot; , &quot;;

                $(&quot;#&quot; + &quot; , &quot;'&quot; , &quot;Titre&quot; , &quot;'&quot; , &quot;).change(function(){
                    var lastchild = $(&quot;#&quot; + &quot; , &quot;'&quot; , &quot;Titre&quot; , &quot;'&quot; , &quot; + &quot; option:last-child&quot;).val();
                    toggleAdditionInfoTextBox(this,lastchild);
                });

                $(&quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;Titre&quot; , &quot;'&quot; , &quot;).click(function (){
                    var lastchild = $(&quot;.&quot; + &quot; , &quot;'&quot; , &quot;Titre&quot; , &quot;'&quot; , &quot; + &quot;_last&quot;).val();
                    toggleAdditionInfoTextBox(this,lastchild);
                });

                function toggleAdditionInfoTextBox(self,lastchildVal) {
                    var current = $(self).val();

                    if (lastchildVal === current) {
                        showAdditionInfoTextBox();
                    } else {
                        hideAdditionInfoTextBox();
                    }
                }

                function showAdditionInfoTextBox() {
                    appendToParent();
                    $(extraInfoSelector).show();
                    $(extraInfoSelector + &quot; INPUT&quot;).focus();
                }

                function hideAdditionInfoTextBox() {
                    appendToParent();
                    $(extraInfoSelector).hide();
                }

                function appendToParent() {
                    var parent = $(extraInfoSelector).parent();
                    $(parent).append($(extraInfoSelector));
                }

                return {
                    showAdditionInfoTextBox:showAdditionInfoTextBox,
                    hideAdditionInfoTextBox:hideAdditionInfoTextBox
                };
            }

           return {
               showAdditionInfoTextBox:{},
               hideAdditionInfoTextBox:{}
           };
       })();

        $(function () {
            var showOptionalText = false;
            var additionalInfoEntered = false;
            var lastQuestionWasSelected= false;
                    //or last value selected
            if (showOptionalText &amp;&amp; (additionalInfoEntered || lastQuestionWasSelected)) {
                func_Titre.showAdditionInfoTextBox();
            }
        });

        function autoCompleteShowAdditionInfo_Titre() {
            var showOptionalText = false;
            if (showOptionalText) {
                func_Titre.showAdditionInfoTextBox();
            }
        }

        function autoCompleteHideAdditionInfoTextBox_Titre() {
            var showOptionalText = false;
            if (showOptionalText) {
                func_Titre.hideAdditionInfoTextBox();
            }
        }

    

                
            

            
                
                    Nom
                
                

                    
                
      
        
        Nom
      
      
    
                
                    
                
            

            
                
                    Prénom
                
                

                    
                
      
        
        Prénom
      
      
    
                
                    
                
            

        

        

        
        
    
    
        
            
        
    
            



            



        
            Email professionnel
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Email professionnel
      
      
    
    
        
            
        
    
            



            



        
            Numéro de téléphone professionnel
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Numéro de téléphone professionnel
      
      
    
    
        
            
        
    
            
            
                



            


                    
                        Mon entreprise
                    

            



            



        
            Nom de l’entreprise
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Nom de l’entreprise
      
      
    
    
        
            
        
    
            



            



        
            Adresse de l’entreprise
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Adresse de l’entreprise
      
      
    
    
        
            
        
    
            



            



        
            Code postal de l&quot; , &quot;'&quot; , &quot;entreprise
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Code postal de l&quot; , &quot;'&quot; , &quot;entreprise
      
      
    
    
        
            
        
    
            



            



        
            Ville de l’entreprise
        
    
        




        
            
        
        
        

        

        
        
    
      
        
        Ville de l’entreprise
      
      
    
    
        
            
        
    
            



            








    


        

    

    
Je m’oppose à ce que mon numéro de téléphone et mon adresse email soient utilisés pour recevoir des offres commerciales d’Hiscox 




    


            
        

		

	
	
	
	
	
	


	

		
    Saved





	



                
                
                

		
			
				


    Saved


             Étape précédente






    var $questionForm = null,
        $quoteBackButton = null,
        $quoteGoBackwardElement = &quot;&lt;input type=\&quot;hidden\&quot; id=\&quot;goBackward\&quot; name=\&quot;goBackward\&quot; value=\&quot;false\&quot; data-ays-ignore=\&quot;true\&quot; />&quot;
        ;

    var backClickHandlerSet = false;

    $(function () {

        &quot;use strict&quot;;

        $quoteBackButton = $(&quot;#backButton&quot;);

        $questionForm = $quoteBackButton
            .parentsUntil(&quot;form&quot;)
            .parent();

        function clearIsDirty() {

            if ($questionForm.length !== 0) {

                $questionForm.remove(&quot;#goBackward&quot;);
            }
        }

        function setIsDirty () {

            if ($questionForm.length !== 0) {

                $questionForm
                    .append($quoteGoBackwardElement)
                    .find(&quot;#goBackward&quot;)
                    .val(true);
            }
        }

        if ($questionForm.length !== 0) {

            // Enable detection of changes to question answers
            $questionForm.areYouSure({ silent : true });
        }

        if ($quoteBackButton.length !== 0 &amp;&amp; $questionForm.length !== 0) {

            $questionForm.on(&quot;dirty.areYouSure&quot;, null, function () {

                setIsDirty();
                $questionForm.addClass(&quot;dirty&quot;);  // This should not be necessary, but there seems to be some unreliability when reading the property later
            });

            if (!backClickHandlerSet) {
                backClickHandlerSet = true;

                $quoteBackButton.on(&quot;click&quot;, null, function () {

                    var $questionForm = $(this)
                        .parentsUntil(&quot;form&quot;)
                        .parent();

                    $questionForm.trigger(&quot;checkform.areYouSure&quot;);

                    if ($questionForm.hasClass(&quot;dirty&quot;)) {
                        // Changes detected to the question answers.  Ensure client side validation is enforced.
                        //$questionForm.submit();
                        saveAnswersFrom(window.location.href, &quot; , &quot;'&quot; , &quot;quoteRef&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/Public/QuickQuoteQuestions?PackageId=12462&amp;quoteRef=Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&amp;createdFromType=NewBusiness&amp;isUpdate=False&amp;pageNumber=2&amp;goBackwards=True&amp;releaseDateOption=NotSet&amp;showSendReferralEmailButton=False&amp;quoteIsReadOnly=False&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SavedFrom&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SaveButton&quot; , &quot;'&quot; , &quot;);
                        // saveAnswers(window.location.href, &quot; , &quot;'&quot; , &quot;quoteRef&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;/Public/QuickQuoteQuestions?PackageId=12462&amp;quoteRef=Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&amp;createdFromType=NewBusiness&amp;isUpdate=False&amp;pageNumber=2&amp;goBackwards=True&amp;releaseDateOption=NotSet&amp;showSendReferralEmailButton=False&amp;quoteIsReadOnly=False&quot; , &quot;'&quot; , &quot;);
                    } else {
                        // No changes were detected to the question answers.  Bypass client side validation and redirect to the previous page.
                        clearIsDirty();
                        location.href = &quot;/Public/QuickQuoteQuestions?PackageId=12462&amp;quoteRef=Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&amp;createdFromType=NewBusiness&amp;isUpdate=False&amp;pageNumber=2&amp;goBackwards=True&amp;releaseDateOption=NotSet&amp;showSendReferralEmailButton=False&amp;quoteIsReadOnly=False&quot;;
                    }
                });
            }
        }
    });


						

								Save
							Étape suivante 
						
				
			
		


                
                
                




                


    .toastr-btn {
        float: right;
        margin-right: 4px;
    }



    


    
        
            
                
                    Your session has expired, you will have to restart your quote
                
                
                    
                        Restart Quote
                    
                
            
        
    



    &quot;use strict&quot;;

    Date.prototype.getUTCTime = function () {
        return (this.getTime() + (this.getTimezoneOffset() * 60 * 60 * 1000)); // Add the offset hours in ms
    }

    Date.prototype.getUTC = function () {
        return new Date(this.getUTCTime());
    }

    Date.__proto__.utc = function () {
        return new Date().getUTCTime();
    }

    Date.__proto__.UTCNow = function () {
        return new Date(Date.utc());
    }

    const sessionWarningCookieName = &quot;__Host-SessionExpirationWarningTime&quot;;
    const minutesBeforeExpirationToWarn = 3;
    const minutesBeforeExpirationToWarnMili = minutesBeforeExpirationToWarn * 60000;
    const isEnabled = true;
    const sessionLengthMili = 900000;

    var sessionWarningTime = new Date();
    var sessionWarningTimer = 0;
    var sessionExpiredTimer = 0;

    var isLoginTypePublic = true

    $(function () {
        if (isEnabled === true) {
            InitialiseSessionAlerts();
        }
    });

    function SetSessionWarningTime(warningMilliseconds) {
        sessionWarningTime = new Date(Date.now() + warningMilliseconds);
    }

    function InitialiseSessionAlerts() {
        var milisecondsTillWarning = sessionLengthMili - minutesBeforeExpirationToWarnMili;

        if (!isLoginTypePublic || (window.location.href.includes(&quot;quoteRef&quot;) || window.location.href.includes(&quot;QuickQuoteQuestions&quot;) || window.location.href.includes(&quot;PreQuoteQuestionsContinue&quot;))) {
            ClearTimers();
            SetSessionWarningTime(milisecondsTillWarning);
            SetTimers(milisecondsTillWarning, sessionLengthMili);
        }
    }

    function AdjustSessionTimers(milisecondsTillWarning) {
        var milisecondsTillExpiration = milisecondsTillWarning + minutesBeforeExpirationToWarnMili;

        ClearTimers();
        SetSessionWarningTime(milisecondsTillWarning);
        SetTimers(milisecondsTillWarning, milisecondsTillExpiration);
    }

    function ClearTimers() {
        window.clearTimeout(sessionWarningTimer);
        window.clearTimeout(sessionExpiredTimer);
    }

    function SetTimers(warningWait, expirationWait) {
        if (warningWait > 0) {
            sessionWarningTimer = window.setTimeout(PromptKeepSessionActive, warningWait);
        }
        sessionExpiredTimer = window.setTimeout(AlertSessionExpired, expirationWait);

    }

    function CheckAndUpateTimers() {
        var validTimeChange = false;
        var dateString = getSessionExpiryFromCookie(sessionWarningCookieName);
        var expirationTime = new Date(dateString);
        var cookieWarningTime = new Date(expirationTime.getTime() - minutesBeforeExpirationToWarnMili);

        if (Date.now() &lt; expirationTime.getTime()) {
            if (cookieWarningTime.getTime() != sessionWarningTime.getTime()) {
                var timeDiff = Math.max(0, (cookieWarningTime.getTime() - Date.now()));

                // Cookie was changed by another tab, need to upate the timers in this tab
                if (timeDiff > 0) {
                    AdjustSessionTimers(timeDiff);
                    validTimeChange = true;
                }
            }
        }

        return validTimeChange;
    }

    function PromptKeepSessionActive() {
        toastr.remove();

        if (CheckAndUpateTimers()) {
            return;
        }

        var common = new Instanda.Common();
        if (isLoginTypePublic) {
            var message = &quot; , &quot;'&quot; , &quot;Your session is about to expire, do you wish to extend it?&quot; , &quot;'&quot; , &quot;;
            var buttonText = &quot; , &quot;'&quot; , &quot;Yes&quot; , &quot;'&quot; , &quot;;
        } else {
            var message = &quot; , &quot;'&quot; , &quot;Your session is about to expire, do you wish to extend it?&quot; , &quot;'&quot; , &quot;;
            var buttonText = &quot; , &quot;'&quot; , &quot;Yes&quot; , &quot;'&quot; , &quot;;
        }


        var encryptedQuoteRef = null;

             encryptedQuoteRef = &quot; , &quot;'&quot; , &quot;Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&quot; , &quot;'&quot; , &quot;;

        toastr.warning(&quot;&lt;div>&lt;button type=&quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot; id=&quot; , &quot;'&quot; , &quot;confirmationButtonYes&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;btn-extend-session btn instanda-button btn btn-primary toastr-btn&quot; , &quot;'&quot; , &quot;>&quot; + buttonText + &quot;&lt;/button>&lt;/div>&quot;, message,
            {
                positionClass: &quot;toast-top-center&quot;,
                timeOut: 0,
                extendedTimeOut: 0,
                preventDuplicates: false,
                closeButton: true,
                allowHtml: true,
                escapeHtml: false,
                onShown: function (toast) {
                    $(&quot;#confirmationButtonYes&quot;).click(function () {
                        common.keepSessionActive(encryptedQuoteRef);
                        InitialiseSessionAlerts();
                        toastr.remove();
                    });
                }
            });
    }

    function AlertSessionExpired() {
        toastr.remove();

        if (CheckAndUpateTimers()) {
            return;
        }

        if (isLoginTypePublic) {
            if (true) {
                $(&quot; , &quot;'&quot; , &quot;#publicLiveSessionExpiredPopupDialog&quot; , &quot;'&quot; , &quot;).modal(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
            } else {
                document.getElementById(&quot; , &quot;'&quot; , &quot;publicLiveSessionExpiredPopupDialog&quot; , &quot;'&quot; , &quot;).style.display = &quot;block&quot;;
            }
        } else {

            var message = &quot; , &quot;'&quot; , &quot;Your session has expired, you will be redirected to the login page&quot; , &quot;'&quot; , &quot;;
            var hideButtonText = &quot; , &quot;'&quot; , &quot;Hide&quot; , &quot;'&quot; , &quot;;
            var loginButtonText = &quot; , &quot;'&quot; , &quot;Login&quot; , &quot;'&quot; , &quot;;

            toastr.error(&quot;&lt;div>&lt;button type=&quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot; id=&quot; , &quot;'&quot; , &quot;confirmationButtonOK&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;btn instanda-button btn btn-primary toastr-btn&quot; , &quot;'&quot; , &quot;>&quot; + hideButtonText + &quot;&lt;/button>&quot; +
                &quot;&lt;button type=&quot; , &quot;'&quot; , &quot;button&quot; , &quot;'&quot; , &quot; id=&quot; , &quot;'&quot; , &quot;loginButtonOK&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;btn instanda-button btn btn-primary toastr-btn&quot; , &quot;'&quot; , &quot;>&quot; + loginButtonText + &quot;&lt;/button>&lt;/div>&quot;,
                message,
                {
                    positionClass: &quot;toast-top-center&quot;,
                    timeOut: 0,
                    extendedTimeOut: 0,
                    preventDuplicates: false,
                    closeButton: true,
                    allowHtml: true,
                    escapeHtml: false,
                    onShown: function (toast) {
                        $(&quot;#confirmationButtonOK&quot;).click(function () {
                            $(&quot;.toast-close-button&quot;).click();
                        });
                        $(&quot;#loginButtonOK&quot;).click(function () {
                            RedirectToLogin();
                        });
                    }
                });
        }

        killSession();
    }

    $(&quot;#publicLiveSessionExpiredRestartQuoteButton&quot;).click(function () {
        RestartQuote();
    });

    function RestartQuote() {
            window.location.href = &quot;/Public/QuickQuoteQuestions?packageId=12462&quot;;
    }

    function CreateCookie(name, value, days) {
        if (days) {
            var date = new Date();
            date.setTime(date.getTime() + (days * 24 * 60 * 60 * 1000));
            var expires = &quot;; expires=&quot; + date.toGMTString();
        } else {
            var expires = &quot;&quot;;
        }

        document.cookie = name + &quot;=&quot; + value + expires + &quot;; path=/&quot;;
    }

    function getSessionExpiryFromCookie(name) {
       const nameEQ = name + &quot;=&quot;;
        const ca = document.cookie.split(&quot; , &quot;'&quot; , &quot;;&quot; , &quot;'&quot; , &quot;);
        for (let i = 0; i &lt; ca.length; i++) {
            let c = ca[i];
            while (c.charAt(0) == &quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;) c = c.substring(1, c.length);
            if (c.indexOf(nameEQ) == 0) {
                let value = c.substring(nameEQ.length, c.length);
                let expiry = new Date(value);
                return expiry;
            };
        }

        // If it can&quot; , &quot;'&quot; , &quot;t find the cookie then return current time (i.e. act like the session just expired)
        return Date.UTCNow();
    }

    function RedirectToLogin() {
        var loginType = &quot; , &quot;'&quot; , &quot;Public&quot; , &quot;'&quot; , &quot;;

        if (loginType == &quot;Client&quot;) {
            window.location.href = &quot;/&quot;;
            return;
        }

        $.ajax({
            type: &quot;GET&quot;,
            url: &quot;/Public/GetSessionLoginUrl?login=&quot; + loginType,
            success: function (data) {
                window.location.href = data;
                return;
            },
            error: function (jqXHR, textStatus, error) {

                var errorMessage = &quot;&quot;;

                if (jqXHR.responseJSON) {
                    if (jqXHR.responseJSON.IsRedirect) {
                        $(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;reinitialize.areYouSure&quot; , &quot;'&quot; , &quot;);
                        window.location.href = jqXHR.responseJSON.RedirectURL;
                        return;
                    } else if (jqXHR.responseJSON.HTMLContent) {
                        $(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;reinitialize.areYouSure&quot; , &quot;'&quot; , &quot;);
                        var htmlPageContent = jqXHR.responseJSON.HTMLContent;
                        document.write(htmlPageContent);
                        document.close();
                        return;
                    } else if (jqXHR.responseJSON.ErrorMessage) {
                        errorMessage = jqXHR.responseJSON.ErrorMessage;
                    } else {
                        errorMessage = jqXHR.responseJSON;
                    }
                } else {
                    errorMessage = JSON.parse(jqXHR.responseText);
                }
            }
        });
    }

    function killSession() {
        $.ajax({
            type: &quot;GET&quot;,
            url: &quot;/Public/AjaxPublicKillSession&quot;,
            success: function (data) { },
            error: function (jqXHR, textStatus, error) {

                var errorMessage = &quot;&quot;;

                if (jqXHR.responseJSON) {
                    if (jqXHR.responseJSON.IsRedirect) {
                       //the user will be redirected later based on the session expiration dialog choice
                        return;
                    } else if (jqXHR.responseJSON.HTMLContent) {
                        $(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;reinitialize.areYouSure&quot; , &quot;'&quot; , &quot;);
                        var htmlPageContent = jqXHR.responseJSON.HTMLContent;
                        document.write(htmlPageContent);
                        document.close();
                        return;
                    } else if (jqXHR.responseJSON.ErrorMessage) {
                        errorMessage = jqXHR.responseJSON.ErrorMessage;
                    } else {
                        errorMessage = jqXHR.responseJSON;
                    }
                } else if (jqXHR.status !== 403) {
                    errorMessage = jqXHR.responseText;
                }

                if (errorMessage !== &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    throw (errorMessage);
                }
            }
        });
    }

    $(document).ajaxSend(function (e, xhr, options) {

        var cookieTimeMili = new Date(getSessionExpiryFromCookie(sessionWarningCookieName)).getTime();
        var expiryTimeMili = cookieTimeMili + minutesBeforeExpirationToWarnMili;
        var currentTimeMili = new Date().getTime();

        var url = options.url;
        if (url != null &amp;&amp; url === &quot;/Public/AjaxPublicKillSession&quot;) {
            return;
        }

        if (expiryTimeMili &lt; currentTimeMili) {
            xhr.abort();
            $(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;reinitialize.areYouSure&quot; , &quot;'&quot; , &quot;);
            window.location.reload();
        }
    });

    $(document).ajaxComplete(function (event, jqXHR, ajaxOptions) {

        var extendSession = true;

        if (typeof jqXHR.statusText !== &quot;undefined&quot; &amp;&amp; jqXHR.statusText == &quot;abort&quot; &amp;&amp; typeof jqXHR.status !== &quot;undefined&quot; &amp;&amp; jqXHR.status == 0) {
            extendSession = false;
        } else if (typeof ajaxOptions.url !== &quot;undefined&quot; &amp;&amp; ajaxOptions.url === &quot;/Public/AjaxPublicKillSession&quot;) {
            extendSession = false;
        } else if (typeof jqXHR.responseJSON !== &quot;undefined&quot; &amp;&amp; typeof jqXHR.responseJSON.notExtendSession !== &quot;undefined&quot; &amp;&amp; typeof jqXHR.responseJSON.notExtendSession === &quot;boolean&quot;) {
            extendSession = !jqXHR.responseJSON.notExtendSession;
        }

        if (extendSession) {
            InitialiseSessionAlerts();
            //we only want to clear toast warnings
            $(&quot; , &quot;'&quot; , &quot;.btn-extend-session&quot; , &quot;'&quot; , &quot;).each(function (i, obj) {
                toastr.remove();
            });
        }
    });


            

            






Votre devis






Pour votre activité


Responsabilité Civile Professionnelle



Protection Juridique





Prime TTC 14,39 €






























Total TTC

14,39
par mois






Changer mon offre






€163,65TTC



€83,33TTC

soit 166,65 € par an



€42,41TTC

soit 169,64 € par an



€14,39TTC

soit 172,64 € par an

            
            




Les données personnelles fournies seront utilisées pour vous adresser le devis que vous sollicitez et pour la gestion de votre contrat d&quot; , &quot;'&quot; , &quot;assurance. Hiscox traite vos données personnelles avec le plus grand soin et ne les vend pas à des tiers à des fins de marketing. En cliquant sur &quot;Étape suivante&quot;, je déclare avoir pris connaissance de la notice d&quot; , &quot;'&quot; , &quot;information relative au traitement de mes données personnelles 










 

            
            
        



        
             
        
        

        

            $(function () {
                PreventDoubleSubmission($(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;));

            });

            window.thousandsSeperator = &quot; , &quot;'&quot; , &quot;&amp;#160;&quot; , &quot;'&quot; , &quot;;
            window.decimalSeperator = &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;;

            var validator = function () {

                var pub = {};

                pub.run = function (form) {
                    form.removeData(&quot; , &quot;'&quot; , &quot;validator&quot; , &quot;'&quot; , &quot;);
                    form.removeData(&quot; , &quot;'&quot; , &quot;unobtrusiveValidation&quot; , &quot;'&quot; , &quot;);
                    form.removeData(&quot; , &quot;'&quot; , &quot;submitted&quot; , &quot;'&quot; , &quot;);

                    $.validator.unobtrusive.parse(form);
                };

                return pub;
            }();

            $(&quot;form&quot;).submit(function (e) {

                var form = $(this);

                validator.run(form);
            });


        






  /* System hack */
  (function() {
    // hack system createAutoCompleteQuestion()
    const oldCreateAutoCompleteQuestion = createAutoCompleteQuestion;
    window.createAutoCompleteQuestion = function() {
      oldCreateAutoCompleteQuestion.apply(this, arguments);

      const questionId = arguments[0];
      const target = $(&quot;#&quot; + questionId).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-input&quot; , &quot;'&quot; , &quot;)[0];
      const observer = new MutationObserver(function(mutationList, observer) {
        for (const mutation of mutationList) {
          if (mutation.type == &quot; , &quot;'&quot; , &quot;childList&quot; , &quot;'&quot; , &quot; &amp;&amp; mutation.addedNodes.length > 0 &amp;&amp; mutation.addedNodes[0].classList.contains(&quot; , &quot;'&quot; , &quot;typeahead&quot; , &quot;'&quot; , &quot;)) {
            $(mutation.target).trigger(&quot; , &quot;'&quot; , &quot;autoComplete.built&quot; , &quot;'&quot; , &quot;);
          }
        }
      });
      observer.observe(target, {childList: true});
    }
  })();


  /* set tippy theme */
  tippy.setDefaultProps({
    &quot; , &quot;'&quot; , &quot;theme&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;custom&quot; , &quot;'&quot; , &quot;
  });


  /* Add class to body for style to fix input zoom for specific device or browser */
  navigator.userAgent.match(/iPhone|iPad|iPod/i) &amp;&amp; document.body.classList.add(&quot; , &quot;'&quot; , &quot;iOS&quot; , &quot;'&quot; , &quot;);
  navigator.userAgent.match(/Safari/i) &amp;&amp; document.body.classList.add(&quot; , &quot;'&quot; , &quot;safari&quot; , &quot;'&quot; , &quot;);


  /**
   * DOM Helpers - define html attributes for relocating elements
   *
   * Attribute names:
   *   - data-append-to
   *   - data-prepend-to
   *   - data-insert-after
   *   - data-insert-before
   *
   * Attribute values:
   *   valid jquery selector string
   */
  (function() {
    function domHelpers() {
      $(&quot; , &quot;'&quot; , &quot;[data-append-to]&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;append-to&quot; , &quot;'&quot; , &quot;).each(function() {
        var selector = $(this).data(&quot; , &quot;'&quot; , &quot;append-to&quot; , &quot;'&quot; , &quot;);
        $(this).appendTo(selector);
      });
      $(&quot; , &quot;'&quot; , &quot;[data-prepend-to]&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;prepend-to&quot; , &quot;'&quot; , &quot;).each(function() {
        var selector = $(this).data(&quot; , &quot;'&quot; , &quot;prepend-to&quot; , &quot;'&quot; , &quot;);
        $(this).prependTo(selector);
      });
      $(&quot; , &quot;'&quot; , &quot;[data-insert-after]&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;insert-after&quot; , &quot;'&quot; , &quot;).each(function() {
        var selector = $(this).data(&quot; , &quot;'&quot; , &quot;insert-after&quot; , &quot;'&quot; , &quot;);
        $(this).insertAfter(selector);
      });
      $(&quot; , &quot;'&quot; , &quot;[data-insert-before]&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;insert-before&quot; , &quot;'&quot; , &quot;).each(function() {
        var selector = $(this).data(&quot; , &quot;'&quot; , &quot;insert-before&quot; , &quot;'&quot; , &quot;);
        $(this).insertBefore(selector);
      });
      $(&quot; , &quot;'&quot; , &quot;[data-toggle-by-variable]&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;toggle-by-variable&quot; , &quot;'&quot; , &quot;).each(function() {
        var name = $(this).data(&quot; , &quot;'&quot; , &quot;toggle-by-variable&quot; , &quot;'&quot; , &quot;);
        $(this).toggle(Boolean(Instanda.Variables[name]));
      });
    }

    $(document)
      .ready(domHelpers)
      .ajaxComplete(domHelpers);
  })();


  /**
   * Util functions under namespace `Instanda.utils`
   *
   * isMobile()
   *   check if it is mobile
   */
  (function() {
    Instanda.utils = Instanda.utils || {};

    Instanda.utils.isMobile = function() {
      return window.outerWidth &lt;= 767;
    }
  })();


  // example: https://hiscoxdesign.instanda.com/Public/QuickQuoteQuestions?PackageId=12462&amp;pageNumber=1&amp;Sector_CHOICE=V%C3%A9hicule%20de%20Tourisme%20avec%20Chauffeur&amp;Turnover_NUM=2000&amp;Activity_CHOICE=Je%20ne%20trouve%20pas%20mon%20activit%C3%A9#debug
  (function() {
    $(window).one(&quot;load&quot;, function () {
      setTimeout(function() {
        const search = new URLSearchParams(window.location.search);
        for (const key of search.keys()) {
          const control = document.getElementById(key);
          if (control &amp;&amp; control.type !== &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) {
            const value = search.get(key);
            $(control).val(value).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);

            if (control.tagName === &quot; , &quot;'&quot; , &quot;SELECT&quot; , &quot;'&quot; , &quot;) {
              $(control).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-input&quot; , &quot;'&quot; , &quot;).one(&quot; , &quot;'&quot; , &quot;autoComplete.built&quot; , &quot;'&quot; , &quot;, function(event) {
                const $input = $(event.currentTarget).find(&quot; , &quot;'&quot; , &quot;.tt-input&quot; , &quot;'&quot; , &quot;);
                const controlId = $input.attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;);
                const value = search.get(controlId);
                $(event.currentTarget.children[0]).find(&quot; , &quot;'&quot; , &quot;.tt-input&quot; , &quot;'&quot; , &quot;).typeahead(&quot; , &quot;'&quot; , &quot;val&quot; , &quot;'&quot; , &quot;, value);
              });
            }
          }
        }
      }, 0);
    });
  }) ();


  /**
   * MDC Builders
   */
  (function() {
    // Templates
    var mdcSelectTempl = document.getElementById(&quot;mdc-select-tmpl&quot;).innerHTML;
    var mdcTextfieldTempl = document.getElementById(&quot;mdc-textfield-tmpl&quot;).innerHTML;
    var mdcSwitchTempl = document.getElementById(&quot;mdc-switch-tmpl&quot;).innerHTML;
    var mdcMenuTempl = document.getElementById(&quot;mdc-menu-tmpl&quot;).innerHTML;

    Mustache.parse(mdcSelectTempl);
    Mustache.parse(mdcTextfieldTempl);
    Mustache.parse(mdcSwitchTempl);
    Mustache.parse(mdcMenuTempl);

    // DEBUG
    function createDebugSwitch() {
      var rendered = Mustache.render(mdcSwitchTempl, {});
      var elem = $(rendered)[0];
      elem.style.cssText=&quot;position:fixed; bottom:15px; right:15px; display:block !important;&quot;;
      document.body.append(elem);

      var component = mdc.switchControl.MDCSwitch.attachTo(elem.querySelector(&quot; , &quot;'&quot; , &quot;.mdc-switch&quot; , &quot;'&quot; , &quot;));
      component.nativeControl_.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function(event) {
        $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).toggleClass(&quot; , &quot;'&quot; , &quot;my-mdc-off&quot; , &quot;'&quot; , &quot;, event.target.checked);
      });
    }
    $(document).ready(function() {
      window.location.hash.indexOf(&quot; , &quot;'&quot; , &quot;debug&quot; , &quot;'&quot; , &quot;) > -1 &amp;&amp; createDebugSwitch();
    });

    // Classes
    class MDCComponentBuilder {
      constructor(ctrlElem, labelText, ctrlContainer, labelContainer, helpContainer = null) {
        this.ctrlElem = ctrlElem;
        this.labelText = this.htmlDecode(labelText);
        this.ctrlContainer = ctrlContainer;
        this.labelContainer = labelContainer;
        this.helpContainer = helpContainer;
        this.template = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        this.mutatedAttributes = [&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;];
      }

      htmlDecode(input) {
        var doc = new DOMParser().parseFromString(input, &quot;text/html&quot;);
        return doc.documentElement.textContent;
      }

      init() {
        this.mdcWrapperElement = null;
        this.mdcComponent = null;

        // @deprecated
        this.helpText = this.getHelpText();
      }

      // @deprecated
      getHelpText() {
        var $dom = $(this.ctrlContainer).find(&quot; , &quot;'&quot; , &quot;.field-validation-error&quot; , &quot;'&quot; , &quot;);
        return $dom.length > 0 ? $dom.html() : &quot;&quot;;
      }

      hideElements() {
        $(this.ctrlContainer).addClass(&quot; , &quot;'&quot; , &quot;my-mdc-hide&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;aria-hidden&quot; , &quot;'&quot; , &quot;, &quot;true&quot;);
        $(this.ctrlElem).attr(&quot; , &quot;'&quot; , &quot;tabindex&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;-1&quot; , &quot;'&quot; , &quot;);
        this.labelContainer &amp;&amp; $(this.labelContainer).addClass(&quot; , &quot;'&quot; , &quot;my-mdc-hide&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;aria-hidden&quot; , &quot;'&quot; , &quot;, &quot;true&quot;);
      }

      isBuilt() {
        return true &amp;&amp; $(this.ctrlContainer).data(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;);
      }

      build() {
        if (this.isBuilt()) return;

        this.prepareComponent();
        this.moveHelpIcon();
        this.syncStyle();
        this.bindEvents();
        this.observeMutation();
        this.hideElements();
        this.insert();
        $(this.ctrlContainer).data(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;, true);
        $(this.mdcWrapperElement).data(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;, true);
      }

      prepareComponent() {
      }

      moveHelpIcon() {
        const $iconContainer = $(this.ctrlContainer).find(&quot; , &quot;'&quot; , &quot;.instanda-responsive-help-icon-container&quot; , &quot;'&quot; , &quot;);
        $iconContainer.length > 0 &amp;&amp; $(this.mdcWrapperElement).append($iconContainer);
      }

      syncStyle() {
        this.mdcWrapperElement.setAttribute(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;, this.ctrlElem.getAttribute(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;) || &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
      }

      bindEvents() {
      }

      observeMutation() {
        var config = { childList: true, attributes: true, subtree: true };
        this.observer = new MutationObserver(function(mutationRecords, observer) {
          var syncStyle = false;
          var rebuild = false;
          mutationRecords.forEach(function(mutationRecord) {
            if (mutationRecord.type == &quot; , &quot;'&quot; , &quot;attributes&quot; , &quot;'&quot; , &quot;) {
              if (mutationRecord.attributeName == &quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;) syncStyle = true;
              if (this.mutatedAttributes.indexOf(mutationRecord.attributeName) > 0) rebuild = true;
            }
            if (mutationRecord.type == &quot; , &quot;'&quot; , &quot;childList&quot; , &quot;'&quot; , &quot;) {
              rebuild = true;
            }
          }.bind(this));

          if (syncStyle) this.syncStyle();
          if (rebuild) this.rebuild();
        }.bind(this))
        this.ctrlElem &amp;&amp; this.observer.observe(this.ctrlElem, config);
        this.helpContainer &amp;&amp; this.observer.observe(this.helpContainer, config);
      }

      destroy() {
        this.observer &amp;&amp; this.observer.disconnect();
        this.mdcComponent &amp;&amp; this.mdcComponent.destroy();
        $(this.mdcWrapperElement).removeData(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;).remove();
        $(this.ctrlContainer).removeData(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;);
      }

      revert() {
        this.destroy();
        $(this.ctrlContainer).removeClass(&quot; , &quot;'&quot; , &quot;my-mdc-hide&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;aria-hidden&quot; , &quot;'&quot; , &quot;);
        $(this.ctrlElem).removeAttr(&quot; , &quot;'&quot; , &quot;tabindex&quot; , &quot;'&quot; , &quot;);
        this.labelContainer &amp;&amp; $(this.labelContainer).removeClass(&quot; , &quot;'&quot; , &quot;my-mdc-hide&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;aria-hidden&quot; , &quot;'&quot; , &quot;, &quot;true&quot;);
      }

      rebuild() {
        this.destroy();
        this.init();
        this.build();
      }

      insert() {
        $(this.ctrlContainer).after(this.mdcWrapperElement);
      }
    }

    class MDCSwitchBuilder {
      constructor(ctrlElems, labelText, ctrlContainer, labelContainer, valueOn=&quot;Yes&quot;, valueOff=&quot;No&quot;) {
        this.ctrlElems = ctrlElems;
        this.labelText = labelText;
        this.ctrlContainer = ctrlContainer;
        this.labelContainer = labelContainer;
        // this.valueOn = valueOn;
        // this.valueOff = valueOff;
        this.ctrlElemOn = $(this.ctrlElems).filter(`[value=&quot;${valueOn}&quot;]`)[0];
        this.ctrlElemOff = $(this.ctrlElems).filter(`[value=&quot;${valueOff}&quot;]`)[0];

        this.template = mdcSwitchTempl;
        this.init();
      }

      init() {
        this.mdcWrapperElement = null;
        this.mdcComponent = null;
        this.observer = null;
      }

      hideElements() {
        $(this.ctrlContainer).addClass(&quot; , &quot;'&quot; , &quot;my-mdc-hide&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;aria-hidden&quot; , &quot;'&quot; , &quot;, &quot;true&quot;);
        $(this.ctrlElems).attr(&quot; , &quot;'&quot; , &quot;tabindex&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;-1&quot; , &quot;'&quot; , &quot;);
        this.labelContainer &amp;&amp; $(this.labelContainer).addClass(&quot; , &quot;'&quot; , &quot;my-mdc-hide&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;aria-hidden&quot; , &quot;'&quot; , &quot;, &quot;true&quot;);
      }

      isBuilt() {
        return true &amp;&amp; $(this.ctrlContainer).data(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;);
      }

      build() {
        if (this.isBuilt()) return;

        this.prepareComponent();
        this.bindEvents();
        this.observeMutation();
        this.hideElements();
        this.insert();
        $(this.ctrlContainer).data(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;, true);
        $(this.mdcWrapperElement).data(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;, true);
      }

      prepareComponent() {
        var data = {}
        data.label = this.labelText;
        data.disabled = (
          !this.ctrlElemOn.checked &amp;&amp; this.ctrlElemOn.disabled
        ) || (
          !this.ctrlElemOff.checked &amp;&amp; this.ctrlElemOff.disabled
        );
        data.checked = this.isChecked();
        var rendered = Mustache.render(this.template, data);
        this.mdcWrapperElement = $(rendered)[0];
        this.mdcComponent = mdc.switchControl.MDCSwitch.attachTo(this.mdcWrapperElement.querySelector(&quot; , &quot;'&quot; , &quot;.mdc-switch&quot; , &quot;'&quot; , &quot;));
      }

      isChecked() {
        return $(this.ctrlElemOn).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;);
      }

      bindEvents() {
        var builder = this;

        this.mdcComponent.nativeControl_.addEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, this._changeHandler.bind(this));

        $(this.ctrlElemOn)
          .on(&quot; , &quot;'&quot; , &quot;click.mdc&quot; , &quot;'&quot; , &quot;, function(event) {
            builder.mdcComponent.foundation_.setChecked(true);
          })

        $(this.ctrlElemOff)
          .on(&quot; , &quot;'&quot; , &quot;click.mdc&quot; , &quot;'&quot; , &quot;, function(event) {
            builder.mdcComponent.foundation_.setChecked(false);
          })
      }

      observeMutation() {
        var config = { childList: true, attributes: true, subtree: true };
        this.observer = new MutationObserver(function(mutationRecords, observer) {
          // var syncStyle = false;
          var rebuild = false;
          mutationRecords.forEach(function(mutationRecord) {
            if (mutationRecord.type == &quot; , &quot;'&quot; , &quot;attributes&quot; , &quot;'&quot; , &quot;) {
              // if (mutationRecord.attributeName != &quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;) syncStyle = true;
              if (mutationRecord.attributeName == &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) rebuild = true;
              if (mutationRecord.attributeName == &quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;) rebuild = true;
            }
            if (mutationRecord.type == &quot; , &quot;'&quot; , &quot;childList&quot; , &quot;'&quot; , &quot;) {
              rebuild = true;
            }
          });

          // if (syncStyle) this.syncStyle();
          if (rebuild) this.rebuild();
        }.bind(this))
        this.ctrlElemOn &amp;&amp; this.observer.observe(this.ctrlElemOn, config);
        this.ctrlElemOff &amp;&amp; this.observer.observe(this.ctrlElemOff, config);
      }

      insert() {
        $(this.ctrlContainer).after(this.mdcWrapperElement);
      }

      destroy() {
        this.mdcComponent.nativeControl_.removeEventListener(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, this._changeHandler.bind(this));
        $(this.ctrlElemOn).off(&quot; , &quot;'&quot; , &quot;click.mdc&quot; , &quot;'&quot; , &quot;);
        $(this.ctrlElemOff).off(&quot; , &quot;'&quot; , &quot;click.mdc&quot; , &quot;'&quot; , &quot;);
        this.mdcComponent.destroy();
        $(this.mdcWrapperElement).removeData(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;).remove();
        $(this.ctrlContainer).removeData(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;);
      }

      rebuild() {
        this.destroy();
        this.init();
        this.build();
      }

      _changeHandler(event) {
        if (event.target.checked) {
          $(this.ctrlElemOn).click();
        } else {
          $(this.ctrlElemOff).click();
        }
      }
    }

    class MDCTextFieldBuilder extends MDCComponentBuilder {
      constructor(ctrlElem, labelText, ctrlContainer, labelContainer, helpContainer) {
        super(ctrlElem, labelText, ctrlContainer, labelContainer, helpContainer);

        this.template = mdcTextfieldTempl;
        this.mutatedAttributes = [&quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;];
        this.init();
      }

      prepareComponent() {
        var data = {}
        data.id = this.ctrlElem.id ? this.ctrlElem.id + &quot; , &quot;'&quot; , &quot;--mdc&quot; , &quot;'&quot; , &quot; : null;
        data.label = this.labelText;
        data.disabled = this.ctrlElem.disabled;
        data.readOnly = this.ctrlElem.readOnly;
        data.pattern = this.ctrlElem.getAttribute(&quot; , &quot;'&quot; , &quot;pattern&quot; , &quot;'&quot; , &quot;);
        data.autocomplete = this.ctrlElem.getAttribute(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;);
        data.value = this.ctrlElem.value;
        data.help = this.helpText;
        var rendered = Mustache.render(this.template, data);
        this.mdcWrapperElement = $(rendered)[0];
        this.mdcComponent = mdc.textField.MDCTextField.attachTo(this.mdcWrapperElement.querySelector(&quot; , &quot;'&quot; , &quot;.mdc-text-field&quot; , &quot;'&quot; , &quot;));
      }

      bindEvents() {
        var builder = this;
        var $input = $(this.ctrlElem);
        var $mdcInput = $(&quot;input&quot;, this.mdcWrapperElement)

        $mdcInput
          .on(&quot;input.mdc&quot;, function(event) {
            $input.val(event.target.value).trigger(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;);
          })
          .on(&quot;blur.mdc&quot;, function(event) {
            $input.trigger(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;);
          })
          .on(&quot;change.mdc&quot;, function(event) {
            $input.trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
          });

        $input
          .on(&quot;change.mdc&quot;, function(event) {
            builder.rebuild();
          })
          .on(&quot;mdc:rebuild&quot;, function(event) {
            builder.rebuild();
          })
      }

      destroy() {
        var $input = $(this.ctrlElem);
        var $mdcInput = $(&quot;input&quot;, this.mdcWrapperElement)

        $mdcInput
          .off(&quot;input.mdc&quot;)
          .off(&quot;blur.mdc&quot;);

        $input.off(&quot;change.mdc&quot;);

        super.destroy();
      }
    }

    class MDCSelectBuilder extends MDCComponentBuilder {
      constructor(ctrlElem, labelText, ctrlContainer, labelContainer) {
        super(ctrlElem, labelText, ctrlContainer, labelContainer);

        this.template = mdcSelectTempl;
        this.init();
      }

      init() {
        super.init();

        this.observer = null;
      }

      prepareComponent() {
        var data = {};
        data.id = this.ctrlElem.id ? this.ctrlElem.id + &quot; , &quot;'&quot; , &quot;--mdc&quot; , &quot;'&quot; , &quot; : null;
        data.label = this.labelText;
        data.selectedText = this.ctrlElem.selectedOptions[0] &amp;&amp; this.ctrlElem.selectedOptions[0].value;
        data.options = this.extractOptions();
        var rendered = Mustache.render(this.template, data);
        this.mdcWrapperElement = $(rendered)[0];
        this.mdcComponent = mdc.select.MDCSelect.attachTo(this.mdcWrapperElement.querySelector(&quot; , &quot;'&quot; , &quot;.mdc-select&quot; , &quot;'&quot; , &quot;));
      }

      bindEvents() {
        var builder = this;
        var $select = $(this.ctrlElem);

        this.mdcComponent.listen(&quot;MDCSelect:change&quot;, function(event) {
          builder.ctrlElem.selectedIndex = event.detail.index;
          builder.ctrlElem.dispatchEvent(new Event(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;));
        })

        $select
          .on(&quot;change.mdc&quot;, function(event) {
            builder.rebuild();
          });
      }

      extractOptions() {
        var options = [];
        $(this.ctrlElem).children(&quot; , &quot;'&quot; , &quot;option&quot; , &quot;'&quot; , &quot;).each(function(index, optionElem) {
          var data = {};
          data.selected = $(optionElem).is(&quot; , &quot;'&quot; , &quot;:selected&quot; , &quot;'&quot; , &quot;);
          data.disabled = optionElem.disabled;
          data.hidden = optionElem.hidden;
          data.value = optionElem.value;
          data.text = data.value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; : $(optionElem).html();
          options.push(data);
        });
        return options;
      }
    }

    class mdcTypeAheadBuilder {
      constructor(ctrlElem, menuElem, labelText, ctrlContainer, labelContainer) {
        this.ctrlElem = ctrlElem;
        this.menuElem = menuElem;
        this.labelText = labelText;
        this.ctrlContainer = ctrlContainer;
        this.labelContainer = labelContainer;
      }

      prepareComponent() {
        var rendered = Mustache.render(mdcTextfieldTempl, {label: this.labelText});
        this.mdcWrapperElement = $(rendered)[0];
        this.mdcComponent = mdc.textField.MDCTextField.attachTo(this.mdcWrapperElement.querySelector(&quot; , &quot;'&quot; , &quot;.mdc-text-field&quot; , &quot;'&quot; , &quot;))

        var menuRendered = Mustache.render(mdcMenuTempl, {});
        this.mdcMenuElement =
        this.mdcMenuComponent = mdc.menu.MDCMenu.attachTo();
      }

      build() {
      }

      insert() {
      }
    }

    // building MDC component.
    var buildMDC = function(scope = document) {

      $(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.instanda-question-hierarchy > select&quot; , &quot;'&quot; , &quot;, scope).each(function(index, selectElem) {
        var $selectContainer = $(selectElem).parent();
        var $labelContainer = $selectContainer.prev(&quot; , &quot;'&quot; , &quot;.instanda-question-label&quot; , &quot;'&quot; , &quot;);
        if ($labelContainer.length === 0) {
          var $labelContainer = $selectContainer.closest(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;).prev(&quot; , &quot;'&quot; , &quot;.instanda-question-label&quot; , &quot;'&quot; , &quot;);
        }
        var labelText = $labelContainer.children(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).html();
        var builder = new MDCSelectBuilder(selectElem, labelText, $selectContainer[0], $labelContainer[0]);
        builder.build();

        var observerConfig = { childList: true };
        var observer = new MutationObserver(function(mutationRecords, observer) {
          mutationRecords.forEach(function(mutationRecord) {
            if (mutationRecord.type == &quot; , &quot;'&quot; , &quot;childList&quot; , &quot;'&quot; , &quot;
            &amp;&amp; mutationRecord.removedNodes[0]
            &amp;&amp; mutationRecord.removedNodes[0].tagName == &quot; , &quot;'&quot; , &quot;SELECT&quot; , &quot;'&quot; , &quot;) {
              builder.revert();
              builder = null;
            }
          });
        });
        $selectContainer &amp;&amp; observer.observe($selectContainer[0], observerConfig);
      });

      $(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;)
        .not(&quot; , &quot;'&quot; , &quot;#question121029&quot; , &quot;'&quot; , &quot;)
        .find(&quot; , &quot;'&quot; , &quot;.form-group > .instanda-question-input&quot; , &quot;'&quot; , &quot;, scope)
        .children(&quot; , &quot;'&quot; , &quot;input[type=&quot;text&quot;], input[type=&quot;number&quot;], input[type=&quot;email&quot;], select&quot; , &quot;'&quot; , &quot;)
        .each(function(index, ctrlElem) {
        var $inputContainer = $(ctrlElem).parent();
        var $labelContainer = $inputContainer.siblings(&quot; , &quot;'&quot; , &quot;.instanda-question-inner-label&quot; , &quot;'&quot; , &quot;);
        if ($labelContainer.length == 0) {
          $labelContainer = $inputContainer.closest(&quot; , &quot;'&quot; , &quot;.questionItem > .instanda-text-question&quot; , &quot;'&quot; , &quot;).prev();
        }
        var labelText =
          ctrlElem.name == &quot; , &quot;'&quot; , &quot;AddressLine1&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;Adresse du local&quot; , &quot;'&quot; , &quot; :
          ctrlElem.name == &quot; , &quot;'&quot; , &quot;AddressLine2&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;Complément d’adresse&quot; , &quot;'&quot; , &quot; :
          ctrlElem.name == &quot; , &quot;'&quot; , &quot;Postcode&quot; , &quot;'&quot; , &quot; ? &quot; , &quot;'&quot; , &quot;Code postal&quot; , &quot;'&quot; , &quot; :
          $labelContainer.children(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).html();
        var $helpContainer = $inputContainer.closest(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;[data-valmsg-for=&quot;&quot; , &quot;'&quot; , &quot;+ctrlElem.id+&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;);

        if ($(ctrlElem).is(&quot; , &quot;'&quot; , &quot;input[type=&quot;text&quot;], input[type=&quot;number&quot;], input[type=&quot;email&quot;]&quot; , &quot;'&quot; , &quot;)) {
          var builder = new MDCTextFieldBuilder(ctrlElem, labelText, $inputContainer[0], $labelContainer[0], $helpContainer[0]);
        }
        if ($(ctrlElem).is(&quot; , &quot;'&quot; , &quot;select&quot; , &quot;'&quot; , &quot;)) {
          var builder = new MDCSelectBuilder(ctrlElem, labelText, $inputContainer[0], $labelContainer[0]);
        }
        builder.build();
      });

      $(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.instanda-number-input, .instanda-text-input&quot; , &quot;'&quot; , &quot;, scope).children(&quot; , &quot;'&quot; , &quot;input[type=&quot;text&quot;], input[type=&quot;number&quot;], input[type=&quot;email&quot;]&quot; , &quot;'&quot; , &quot;).each(function(index, ctrlElem) {
        var $inputContainer = $(ctrlElem).closest(&quot; , &quot;'&quot; , &quot;.questionItem > .instanda-text-question&quot; , &quot;'&quot; , &quot;);
        var $labelContainer = $inputContainer.prev();
        var labelText = $labelContainer.children(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).html();
        var $helpContainer = $(ctrlElem).closest(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;[data-valmsg-for=&quot;&quot; , &quot;'&quot; , &quot;+ctrlElem.id+&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;);

        var builder = new MDCTextFieldBuilder(ctrlElem, labelText, $inputContainer[0], $labelContainer[0], $helpContainer[0]);
        builder.build();
      });

      $(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;).
      not(&quot; , &quot;'&quot; , &quot;#question119074&quot; , &quot;'&quot; , &quot;).
      find(&quot; , &quot;'&quot; , &quot;.instanda-question-parent-yes-no&quot; , &quot;'&quot; , &quot;, scope).each(function(index, question) {
        var $ctrlContainer = $(question).find(&quot; , &quot;'&quot; , &quot;> .instanda-text-question&quot; , &quot;'&quot; , &quot;);
        var $labelContainer = $ctrlContainer.prev();
        var labelText = $labelContainer.children(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).html();
        var $ctrlElems = $ctrlContainer.find(&quot; , &quot;'&quot; , &quot;.radio-inline > input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;);
        // convert to switch control if 2 radios input found.
        if ($ctrlElems.length == 2) {
          var builder = new MDCSwitchBuilder($ctrlElems, labelText, $ctrlContainer[0], $labelContainer[0]);
          builder.build();
        }
      });

      var selectors = [
        &quot; , &quot;'&quot; , &quot;#question119728&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;#question119732&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;#question119735&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;#question119738&quot; , &quot;'&quot; , &quot;
      ];
      $(selectors.join(&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;)).each(function(index, coverElem) {
        // &lt;div id=&quot;&quot;>
        //  &lt;div class=&quot;instanda-questionHeader&quot;>&lt;h3>&lt;div>label&lt;/div>&lt;/h3>&lt;/div>
        //  &lt;div>
        //    &lt;input type=&quot;radio&quot; value=&quot;${on}&quot;>
        //    &lt;input type=&quot;radio&quot; value=&quot;${off}&quot;>
        //  &lt;/div>
        // &lt;/div>
        var $labelContainer = $(coverElem).children(&quot; , &quot;'&quot; , &quot;.instanda-questionHeader&quot; , &quot;'&quot; , &quot;)
        var $ctrlContainer = $labelContainer.next();
        var labelText = $labelContainer.find(&quot; , &quot;'&quot; , &quot;h3 > div&quot; , &quot;'&quot; , &quot;).html();
        var $ctrlElems = $ctrlContainer.find(&quot; , &quot;'&quot; , &quot;input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;);
        var builder = new MDCSwitchBuilder($ctrlElems, labelText, $ctrlContainer[0], $labelContainer[0], $ctrlElems[0].value, $ctrlElems[1].value);
        builder.build();
      });

      // $(&quot; , &quot;'&quot; , &quot;div&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;input[type=&quot;text&quot;], input[type=&quot;email&quot;]&quot; , &quot;'&quot; , &quot;).not(&quot; , &quot;'&quot; , &quot;[name*=&quot;OtherAnswer&quot;]&quot; , &quot;'&quot; , &quot;).not(&quot; , &quot;'&quot; , &quot;.focus-class-processed&quot; , &quot;'&quot; , &quot;).each(function() {
      //   var $input = $(this);

      //   $input.addClass(&quot; , &quot;'&quot; , &quot;focus-class-processed&quot; , &quot;'&quot; , &quot;);

      //   // -- HTML structure scenario 1
      //   var $label = $input.closest(&quot; , &quot;'&quot; , &quot;.instanda-question-input&quot; , &quot;'&quot; , &quot;).siblings(&quot; , &quot;'&quot; , &quot;div[class*=&quot;-label&quot;]&quot; , &quot;'&quot; , &quot;);

      //   // -- HTML structure scenario 2
      //   if ($label.length == 0) {
      //     $label = $input.parent().parent().parent(&quot; , &quot;'&quot; , &quot;.instanda-text-question.row&quot; , &quot;'&quot; , &quot;).siblings(&quot; , &quot;'&quot; , &quot;div[class*=&quot;-label&quot;]&quot; , &quot;'&quot; , &quot;)
      //   }

      // // -- HTML structure scenario 3
      //   if ($label.length == 0) {
      //     $label = $input.closest(&quot; , &quot;'&quot; , &quot;.instanda-text-question.row&quot; , &quot;'&quot; , &quot;).siblings(&quot; , &quot;'&quot; , &quot;div[class*=&quot;-label&quot;]&quot; , &quot;'&quot; , &quot;);
      //   }

      //   if ($label.length > 0) {
      //     processTextInput($input, $label);
      //   }
      // })
    }

    // hack system functions
    var oldAddMultiItemByClone = addMultiItemByClone;
    window.addMultiItemByClone = function() {
      oldAddMultiItemByClone.apply(this, arguments);

      var here = arguments[1];

      if (arguments[7] == &quot; , &quot;'&quot; , &quot;Activity_MI&quot; , &quot;'&quot; , &quot;) {
        // remove all wrapper have not been processed by builder.
        $(&quot; , &quot;'&quot; , &quot;#innerBody .mdc-component-wrapper&quot; , &quot;'&quot; , &quot;).each(function(index, wrapperElem) {
          if (!$(wrapperElem).data(&quot; , &quot;'&quot; , &quot;builder-processed&quot; , &quot;'&quot; , &quot;)) {
            $(wrapperElem).remove();
          }
        })

        processItem();
      }

      // build the mdc for controls have not been processed by builder.
      $(document)
        .ready(buildMDC)
        .ajaxComplete(buildMDC);
    }

    function processItem() {
      $(&quot; , &quot;'&quot; , &quot;#insertActivity_MIMultiItemsHere > .instanda-multi-item&quot; , &quot;'&quot; , &quot;).each(function() {
        var thisItem = this;
        var clickHandler = function(event) {
          if (event.target.value == &quot;Yes&quot;) {
            $(thisItem).addClass(&quot; , &quot;'&quot; , &quot;manual-selected&quot; , &quot;'&quot; , &quot;);
          }
          if (event.target.value == &quot;No&quot;) {
            $(thisItem).removeClass(&quot; , &quot;'&quot; , &quot;manual-selected&quot; , &quot;'&quot; , &quot;);
          }
        };
        $(thisItem).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.radio-inline > input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;, clickHandler);
        $(thisItem).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.radio-inline > input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;, clickHandler);
      });
    }

    Instanda.mdc = Instanda.mdc || {};
    Instanda.mdc.MDCComponentBuilder = MDCComponentBuilder;
    Instanda.mdc.MDCTextFieldBuilder = MDCTextFieldBuilder;
    Instanda.mdc.MDCSelectBuilder = MDCSelectBuilder;
    Instanda.mdc.MDCSwitchBuilder = MDCSwitchBuilder;
    Instanda.mdc.build = buildMDC;
  })();


  /**
   * MDC Builders
   *
   * These builders will convert ordinary Instanda questions to material design component (MDC).
   * MDC script and HTML templates are loaded in &lt;head>.
   *
   * The interactions on MDC will sync to the cooresponding Instanda questions for system functions.
   * The builder will hide those Instanda questions after MDC built.
   *
   * Add `#debug` to the url will enable the toggle button at bottom right, it can toggle the Instanda questions
   * and MDC for debugging.
   *
   * This snippet provides a initial function `Instanda.mdc.build()`
   */
  (function() {
    // do nothing if there is no mdc builders
    if (!Instanda.mdc) return;

    $(document)
      .ready(Instanda.mdc.build)
      .ajaxComplete(function() { Instanda.mdc.build(document); });
  })();


  /**
   * Multi-step form
   *
   * It is a factory object for spliting form into multiple steps in one page
   */
  (function() {
    var multiStepForm = function(elem) {
      this._root = this._getElemFromArgument(elem);
      this._current = 0;
      this._forms = [];
    }
    multiStepForm.prototype.setBackBtn = function(elem) {
      var e = this._getElemFromArgument(elem);
      this.backBtn = e;
    };
    multiStepForm.prototype.setContinueBtn = function(elem) {
      var e = this._getElemFromArgument(elem);
      this.continueBtn = e;
    };
    multiStepForm.prototype.setPreviousBtn = function(elem, reference, method = &quot; , &quot;'&quot; , &quot;appendTo&quot; , &quot;'&quot; , &quot;) {
      var e = this._getElemFromArgument(elem);
      var ref = this._getElemFromArgument(reference);
      this.previousBtn = e;
      this.previousBtn &amp;&amp; ref &amp;&amp; this.previousBtn[method](ref);
    };
    multiStepForm.prototype.setNextBtn = function(elem, reference, method = &quot; , &quot;'&quot; , &quot;appendTo&quot; , &quot;'&quot; , &quot;) {
      var e = this._getElemFromArgument(elem);
      var ref = this._getElemFromArgument(reference);
      this.nextBtn = e;
      this.nextBtn &amp;&amp; ref &amp;&amp; this.nextBtn[method](ref);
    };
    multiStepForm.prototype.addForm = function(elem) {
      var e = this._getElemFromArgument(elem);
      e.length == 0 &amp;&amp; typeof elem == &quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot; &amp;&amp; console.warn(&quot; , &quot;'&quot; , &quot;MultiStepForm: cannot find form with selector &quot; , &quot;'&quot; , &quot; + elem);
      return e.length > 0 &amp;&amp; this._forms.push(e) || false;
    };
    multiStepForm.prototype.start = function() {
      // do nothing when no forms defined.
      if (this._forms.length == 0) {
        return;
      }

      var self = this;

      // display first form by default
      $.each(this._forms, function(index, form) {
        if (index > 0) {
          self._hideElem(form);
        }
      });

      self._bindEvents();
      self._displayButtons();
    };
    multiStepForm.prototype._getElemFromArgument = function(elem) {
      if (typeof elem === &quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;) {
        return $(elem);
      }
      if (elem instanceof jQuery) {
        return elem;
      }
      return null;
    }
    multiStepForm.prototype._hideElem = function(elem) {
      var e = this._getElemFromArgument(elem);
      e &amp;&amp; e.hide();
    };
    multiStepForm.prototype._showElem = function(elem) {
      var e = this._getElemFromArgument(elem);
      e &amp;&amp; e.show();
    };
    multiStepForm.prototype._bindEvents = function() {
      var self = this;
      self.nextBtn.on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function (e) { self._gotoNextForm() });
      self.previousBtn.on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function (e) { self._gotoPreviousForm() });
    };
    multiStepForm.prototype._gotoNextForm = function() {
      if (!this._isLastStep()) {
        this._hideElem(this._forms[this._current]);
        this._next();
        this._showElem(this._forms[this._current]);
        this._displayButtons();
      }
    };
    multiStepForm.prototype._gotoPreviousForm = function() {
      if (!this._isFirstStep()) {
        this._hideElem(this._forms[this._current]);
        this._previous();
        this._showElem(this._forms[this._current]);
        this._displayButtons();
      }
    };
    multiStepForm.prototype._isFirstStep = function() {
      return this._current == 0;
    };
    multiStepForm.prototype._isLastStep = function() {
      return this._current == this._forms.length - 1;
    };
    multiStepForm.prototype._next = function() {
      this._current++;
    };
    multiStepForm.prototype._previous = function() {
      this._current--;
    };
    multiStepForm.prototype._displayButtons = function() {
      if (this._isFirstStep()) {
        this._hideElem(this.previousBtn);
        this._showElem(this.backBtn);
      }
      if (!this._isFirstStep()) {
        this._showElem(this.previousBtn);
        this._hideElem(this.backBtn);
      }
      if (this._isLastStep()) {
        this._hideElem(this.nextBtn);
        this._showElem(this.continueBtn);
      }
      if (!this._isLastStep()) {
        this._showElem(this.nextBtn);
        this._hideElem(this.continueBtn);
      }
    };

    Instanda.multiStepForm = multiStepForm;
  })();


  /**
   * Create tooltip with help text content
   */
  (function() {
    var buildOverlay = function() {
      $(&quot; , &quot;'&quot; , &quot;.instanda-question-item&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;overlay-popup&quot; , &quot;'&quot; , &quot;).each(function(index, item) {
        var $icon = $(&quot; , &quot;'&quot; , &quot;.instanda-responsive-help-icon-container a&quot; , &quot;'&quot; , &quot;, item).removeAttr(&quot; , &quot;'&quot; , &quot;title data-target&quot; , &quot;'&quot; , &quot;);
        var $content = $(&quot; , &quot;'&quot; , &quot;.instanda-responsive-help-text > div&quot; , &quot;'&quot; , &quot;, item);

        if ($icon.length > 0 &amp;&amp; $content.length > 0) {
          var cls = $content[0].getAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;).replace(/col-\S*/g, &quot;&quot;).trim().replace(/\s+/g, &quot; &quot;);
          $content[0].setAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;, cls);
          tippy($icon[0], {
            content: $content[0]
          });
        }
      });
    };

    var construct = function() {
      buildOverlay();
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  $(document).ready(function () {
    //--- Show relevant cover options depending on trade
    if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quickquotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == 2) {
      switch(Instanda.Variables.TradeIdentifier_CHOICE) {
        case &quot; , &quot;'&quot; , &quot;PI + Office&quot; , &quot;'&quot; , &quot;:
          $(&quot; , &quot;'&quot; , &quot;input#InsuranceType_CHOICE_Voslocaux, input#InsuranceType_CHOICE_Votreactivitvoslocaux&quot; , &quot;'&quot; , &quot;).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).show();
          break;
        case &quot; , &quot;'&quot; , &quot;PI Only&quot; , &quot;'&quot; , &quot;:
          $(&quot; , &quot;'&quot; , &quot;input#InsuranceType_CHOICE_Voslocaux, input#InsuranceType_CHOICE_Votreactivitvoslocaux&quot; , &quot;'&quot; , &quot;).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).hide();
          break;
        default:
          $(&quot; , &quot;'&quot; , &quot;input#InsuranceType_CHOICE_Voslocaux, input#InsuranceType_CHOICE_Votreactivitvoslocaux&quot; , &quot;'&quot; , &quot;).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).hide();
      }
    }

    $(&quot; , &quot;'&quot; , &quot;header.navbar&quot; , &quot;'&quot; , &quot;).prependTo(&quot; , &quot;'&quot; , &quot;#innerBody&quot; , &quot;'&quot; , &quot;);
  }); // document.ready


  /**
   * Quick Quote Questions (Page 1)
   */
  (function() {
    // do nothing if it is not quick quote questions page 1.
    if ((Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;quickquotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;quickquotequestionscontinue&quot; , &quot;'&quot; , &quot;)
      || Instanda.Variables.PageNumber != 1) return;

    var createMultiStepForm = function() {
      if (Instanda.utils.isMobile()) {
        $(&quot; , &quot;'&quot; , &quot;#innerBody form&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;multiStepForm&quot; , &quot;'&quot; , &quot;).each(function() {
          var multiStep = new Instanda.multiStepForm();
          multiStep.addForm(&quot; , &quot;'&quot; , &quot;#question119038, #question120329&quot; , &quot;'&quot; , &quot;);
          multiStep.addForm(&quot; , &quot;'&quot; , &quot;#question118945&quot; , &quot;'&quot; , &quot;);
          multiStep.setContinueBtn(&quot; , &quot;'&quot; , &quot;[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;);
          multiStep.setBackBtn(&quot; , &quot;'&quot; , &quot;#backButton&quot; , &quot;'&quot; , &quot;);
          multiStep.setNextBtn(&quot; , &quot;'&quot; , &quot;&lt;a class=&quot;btn btn-primary&quot;>Étape suivante&lt;/a>&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;insertAfter&quot; , &quot;'&quot; , &quot;);
          multiStep.setPreviousBtn(&quot; , &quot;'&quot; , &quot;&lt;a class=&quot;btn btn-primary&quot;>Étape précédente&lt;/a>&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;pull-left&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;).insertBefore(multiStep.continueBtn.parent(&quot; , &quot;'&quot; , &quot;.pull-right&quot; , &quot;'&quot; , &quot;)), &quot; , &quot;'&quot; , &quot;appendTo&quot; , &quot;'&quot; , &quot;);
          multiStep.start();
        });
      }
    }

    // edit html for ease of style
    var editDom = function() {
      $(&quot; , &quot;'&quot; , &quot;#insertActivity_MIMultiItemsHere&quot; , &quot;'&quot; , &quot;).children(&quot; , &quot;'&quot; , &quot;.container&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;container&quot; , &quot;'&quot; , &quot;).children(&quot; , &quot;'&quot; , &quot;.row&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;);

      $(&quot; , &quot;'&quot; , &quot;#insertActivity_MIMultiItemsHere&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;[class^=&quot;col-&quot;], [class*=&quot; col-&quot;]&quot; , &quot;'&quot; , &quot;).each(function() {
        var cls = this.getAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;).replace(/col-\S*/g, &quot;&quot;).trim().replace(/\s+/g, &quot; &quot;);
        this.setAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;, cls);
      });

      $(&quot; , &quot;'&quot; , &quot;#multiItemSummary_119677&quot; , &quot;'&quot; , &quot;).children(&quot; , &quot;'&quot; , &quot;.container&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;container&quot; , &quot;'&quot; , &quot;).children(&quot; , &quot;'&quot; , &quot;.row&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;row&quot; , &quot;'&quot; , &quot;);

      $(&quot; , &quot;'&quot; , &quot;#multiItemSummary_119677&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;[class^=&quot;col-&quot;], [class*=&quot; col-&quot;]&quot; , &quot;'&quot; , &quot;).each(function() {
        var cls = this.getAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;).replace(/col-\S*/g, &quot;&quot;).trim().replace(/\s+/g, &quot; &quot;);
        this.setAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;, cls);
      });
    }

    // set min effective change date
    var setMinStartDate = function() {
      var $startDateInput = $(&quot; , &quot;'&quot; , &quot;#EffectiveChangeDate&quot; , &quot;'&quot; , &quot;);

      $startDateInput.length > 0 &amp;&amp; $startDateInput.once(&quot; , &quot;'&quot; , &quot;setMinStartDate&quot; , &quot;'&quot; , &quot;).each(function(elem) {
        var DatePicker = $startDateInput.parent(&quot; , &quot;'&quot; , &quot;.instanda-question-date&quot; , &quot;'&quot; , &quot;).data(&quot;DateTimePicker&quot;);

        if (DatePicker) {
          // bootstrap-datepicker
          DatePicker.minDate(moment().startOf(&quot; , &quot;'&quot; , &quot;d&quot; , &quot;'&quot; , &quot;));
        } else {
          // native
          $startDateInput[0].min = moment().startOf(&quot; , &quot;'&quot; , &quot;d&quot; , &quot;'&quot; , &quot;).format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;);
        }
      });
    }

    // Populate &quot;Je ne trouve pas mon activité&quot; when the checkbox is checked
    var populateCheckbox = function() {
      const $checkbox = $(&quot; , &quot;'&quot; , &quot;#MissingActivity_CONF&quot; , &quot;'&quot; , &quot;);
      const $sector = $(&quot; , &quot;'&quot; , &quot;#Sector_CHOICE&quot; , &quot;'&quot; , &quot;);
      const $activity = $(&quot; , &quot;'&quot; , &quot;#Activity_CHOICE&quot; , &quot;'&quot; , &quot;);

      const cannotFound = &quot; , &quot;'&quot; , &quot;Je ne trouve pas mon activité&quot; , &quot;'&quot; , &quot;;

      // bind autocomplete event after it built
      $(&quot; , &quot;'&quot; , &quot;#Activity_CHOICE&quot; , &quot;'&quot; , &quot;).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;autoComplete.built&quot; , &quot;'&quot; , &quot;, function(event) {
        $(event.currentTarget.children[0]).on(&quot; , &quot;'&quot; , &quot;typeahead:change typeahead:select&quot; , &quot;'&quot; , &quot;, function() {
          if ($(&quot; , &quot;'&quot; , &quot;.tt-input&quot; , &quot;'&quot; , &quot;, this).val() === cannotFound) {
            $checkbox.prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, true).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
          }
        })
      });
      // make activity field readonly when checkbox checked
      $checkbox.on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
        const $activity = $(&quot; , &quot;'&quot; , &quot;#Activity_CHOICE&quot; , &quot;'&quot; , &quot;);
        if ($(this).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
          $activity.val(cannotFound).trigger(&quot; , &quot;'&quot; , &quot;input&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;readOnly&quot; , &quot;'&quot; , &quot;, true);
        } else {
          $activity.val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;readOnly&quot; , &quot;'&quot; , &quot;, false);
        }
      })
      $sector.on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
        $checkbox
          .prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false)
          .parent()
          .removeClass(&quot; , &quot;'&quot; , &quot;ticked&quot; , &quot;'&quot; , &quot;);
        $checkbox.closest(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;).toggle($(this).val() !== &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
      })

      $checkbox.closest(&quot; , &quot;'&quot; , &quot;.questionItem&quot; , &quot;'&quot; , &quot;).toggle($sector.val() !== &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
    }

    var construct = function() {
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quickquotequestions&quot; , &quot;'&quot; , &quot;) {
        createMultiStepForm();
      }
      editDom();
      setMinStartDate();
      populateCheckbox();
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Quick Quote Questions (Page 2).
   */
  (function() {
    // do nothing if it is not quick quote questions page 2.
    if (Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;quickquotequestions&quot; , &quot;'&quot; , &quot; || Instanda.Variables.PageNumber != 2) return;

    var removeSelected = function() {
      $(&quot; , &quot;'&quot; , &quot;#question118955 .instanda-question-choice label.instanda-selected&quot; , &quot;'&quot; , &quot;).each(function(index, elem) {
        $(elem).removeClass(&quot; , &quot;'&quot; , &quot;instanda-selected&quot; , &quot;'&quot; , &quot;);
        var interval = setInterval(function() {
          var $input = $(&quot; , &quot;'&quot; , &quot;input:checked&quot; , &quot;'&quot; , &quot;, elem);
          if ($input.prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;)) {
            $(&quot; , &quot;'&quot; , &quot;input:checked&quot; , &quot;'&quot; , &quot;, elem).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, false);
          }
          else {
            clearInterval(interval);
          }
        }, 400);
      });
    }

    var triggerContinue = function() {
      $(&quot; , &quot;'&quot; , &quot;#question118955 input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;triggerContinue&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
        $(&quot; , &quot;'&quot; , &quot;button[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;).click();
      })
    }

    var _removeChoices = function() {
      //-----Map activities to cover choices on quick quote questions page 2 - PI vs Office vs both
      var option_pi = $(&quot; , &quot;'&quot; , &quot;input#InsuranceType_CHOICE_Votreactivitprofessionnelle&quot; , &quot;'&quot; , &quot;).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-choice&quot; , &quot;'&quot; , &quot;);
      var option_office = $(&quot; , &quot;'&quot; , &quot;input#InsuranceType_CHOICE_Voslocaux&quot; , &quot;'&quot; , &quot;).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-choice&quot; , &quot;'&quot; , &quot;);
      var option_both = $(&quot; , &quot;'&quot; , &quot;input#InsuranceType_CHOICE_Votreactivitvoslocaux&quot; , &quot;'&quot; , &quot;).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-choice&quot; , &quot;'&quot; , &quot;);

      if ( Instanda.Variables.TradeIdentifier_CHOICE == &quot; , &quot;'&quot; , &quot;PI Only&quot; , &quot;'&quot; , &quot; ) {
        option_office.remove();
        option_both.remove();
      } else if ( Instanda.Variables.TradeIdentifier_CHOICE == &quot; , &quot;'&quot; , &quot;Office Only&quot; , &quot;'&quot; , &quot; ) {
        option_pi.remove();
        option_both.remove();
      }
    }

    var buildSwiper = function() {
      $(&quot; , &quot;'&quot; , &quot;#question118955&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;buildSwiper&quot; , &quot;'&quot; , &quot;).each(function(elem) {
        _removeChoices();

        var $slides = $(&quot; , &quot;'&quot; , &quot;.instanda-question-choice&quot; , &quot;'&quot; , &quot;, elem).addClass(&quot; , &quot;'&quot; , &quot;swiper-slide&quot; , &quot;'&quot; , &quot;);
        var $wrapper = $slides.parent().addClass(&quot; , &quot;'&quot; , &quot;swiper-wrapper&quot; , &quot;'&quot; , &quot;).wrap(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;my-swiper-container&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);
        $wrapper.parent().append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;swiper-button-prev&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);
        $wrapper.parent().append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;swiper-button-next&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);

        var swiperOption = {
          watchOverflow: true,
          slidesPerView: Instanda.utils.isMobile() ? 1 : $slides.length,
          navigation: {
            nextEl: &quot; , &quot;'&quot; , &quot;.swiper-button-next&quot; , &quot;'&quot; , &quot;,
            prevEl: &quot; , &quot;'&quot; , &quot;.swiper-button-prev&quot; , &quot;'&quot; , &quot;,
          },
        }
        var swiper = new Swiper(&quot; , &quot;'&quot; , &quot;.my-swiper-container&quot; , &quot;'&quot; , &quot;, swiperOption);
      });
    }

    var construct = function() {
      removeSelected();
      triggerContinue();
      buildSwiper();
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Quick Quote Questions (Page 3)
   */
  (function() {
    // do nothing if it is not quick quote questions page 3.
    if ((Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;quickquotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;quickquotequestionscontinue&quot; , &quot;'&quot; , &quot;)
      || Instanda.Variables.PageNumber != 3) return;

    var createMultiStepForm = function() {
      if (Instanda.utils.isMobile()) {
        $(&quot; , &quot;'&quot; , &quot;#innerBody form&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;multiStepForm&quot; , &quot;'&quot; , &quot;).each(function() {
          var multiStep = new Instanda.multiStepForm();
          multiStep.addForm(&quot; , &quot;'&quot; , &quot;.instanda-quick-quote-questions > .instanda-questionList > .instanda-well:nth-child(2)&quot; , &quot;'&quot; , &quot;);
          multiStep.addForm(&quot; , &quot;'&quot; , &quot;.instanda-quick-quote-questions > .instanda-questionList > .instanda-well:nth-child(3)&quot; , &quot;'&quot; , &quot;);
          multiStep.setContinueBtn(&quot; , &quot;'&quot; , &quot;[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;);
          multiStep.setBackBtn(&quot; , &quot;'&quot; , &quot;#backButton&quot; , &quot;'&quot; , &quot;);
          multiStep.setNextBtn(&quot; , &quot;'&quot; , &quot;&lt;a class=&quot;btn btn-primary&quot;>Étape suivante&lt;/a>&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;insertAfter&quot; , &quot;'&quot; , &quot;);
          multiStep.setPreviousBtn(&quot; , &quot;'&quot; , &quot;&lt;a class=&quot;btn btn-primary&quot;>Étape précédente&lt;/a>&quot; , &quot;'&quot; , &quot;, $(multiStep.continueBtn.parent(&quot; , &quot;'&quot; , &quot;.pull-right&quot; , &quot;'&quot; , &quot;)).siblings(&quot; , &quot;'&quot; , &quot;.pull-left&quot; , &quot;'&quot; , &quot;), &quot; , &quot;'&quot; , &quot;appendTo&quot; , &quot;'&quot; , &quot;);
          multiStep.start();
        });
      }
    }

    var addAutoComplete = function() {
      $(&quot; , &quot;'&quot; , &quot;#EnterpriseName_TXT&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;organization&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#AddressLine1&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;address-line1&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#AddressLine2&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;address-line2&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#City&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;address-level2&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#Postcode118934&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;postal-code&quot; , &quot;'&quot; , &quot;);
    }

    var construct = function() {
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quickquotequestions&quot; , &quot;'&quot; , &quot;) {
        createMultiStepForm();
      }
      addAutoComplete();
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Quick Quote Questions (Page 4)
   */
  (function() {
    if ((Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quickquotequestions&quot; , &quot;'&quot; , &quot; || Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quickquotequestionscontinue&quot; , &quot;'&quot; , &quot;)
    &amp;&amp; Instanda.Variables.PageNumber == 4) {

      // Fix the missing leading 0 on postcode field
      const fixPostCode = function() {
        const field = $(&quot; , &quot;'&quot; , &quot;#Postcode_FormB_NUM&quot; , &quot;'&quot; , &quot;);
        let value = field.val().replace(/\s+/,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;)
        if (value.match(/^\d+$/)) {
          value = value.padStart(5, 0);
        }
        field.val(value).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
      }

      // Auto fill activity choice to Form B
      const autoFillActivityChoice = function() {
        setTimeout(function() {
          $(&quot; , &quot;'&quot; , &quot;#Activity_CHOICE_FormB_TXT&quot; , &quot;'&quot; , &quot;).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;autoComplete.built&quot; , &quot;'&quot; , &quot;, function(event) {
            $(event.currentTarget.children[0]).find(&quot; , &quot;'&quot; , &quot;.tt-input&quot; , &quot;'&quot; , &quot;).typeahead(&quot; , &quot;'&quot; , &quot;val&quot; , &quot;'&quot; , &quot;, Instanda.Variables.Activity_CHOICE);
          });
          $(&quot; , &quot;'&quot; , &quot;#Sector_CHOICE_FormB_TXT&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.Sector_CHOICE).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
        }, 1000)
      }

      const populateTurnoverValue = function() {
        setTimeout(function() {
          $(&quot; , &quot;'&quot; , &quot;#Turnover_FormB_Num&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.Turnover_NUM).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
        }, 1000)
      }

      $(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, autoFillActivityChoice);
      $(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, populateTurnoverValue);
      $(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, fixPostCode);

      // $(document).ready(function() {
      //   autoFillActivityChoice();
      //   populateTurnoverValue();
      //   fixPostCode();
      // });
    }
  })();


  /**
   * Quick Quote (Page 1).
   */
  (function() {
    // do nothing if it is not quick quote page 1.
    if (Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;quickquote&quot; , &quot;'&quot; , &quot; || Instanda.Variables.PageNumber != 1) return;

    // Move radio button to each plan
    var moveRadioBtn = function() {
      $(&quot; , &quot;'&quot; , &quot;#PackSelect_Public_txt_Essential&quot; , &quot;'&quot; , &quot;).appendTo(&quot; , &quot;'&quot; , &quot;.cover-01 .cover-content&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#PackSelect_Public_txt_Equilibre&quot; , &quot;'&quot; , &quot;).appendTo(&quot; , &quot;'&quot; , &quot;.cover-02 .cover-content&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#PackSelect_Public_txt_Integral&quot; , &quot;'&quot; , &quot;).appendTo(&quot; , &quot;'&quot; , &quot;.cover-03 .cover-content&quot; , &quot;'&quot; , &quot;);
    }

    // disable continue button if no radio checked
    var disableContinueBtn = function() {
      var $radios = $(&quot; , &quot;'&quot; , &quot;.cover input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;);
      var $btn = $(&quot; , &quot;'&quot; , &quot;#continueButton&quot; , &quot;'&quot; , &quot;);
      var hasChecked = false;
      if ($btn.length > 0) {
        $radios.each(function(index, radio) {
          hasChecked = hasChecked || radio.checked;
        });
        $btn[0].disabled = !hasChecked;
      }
    }

// sync items across covers in 3 boxes
    var syncCoverItems = function() {
      var $list = $(&quot; , &quot;'&quot; , &quot;.cover-list&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;syncCoverItems&quot; , &quot;'&quot; , &quot;);
      if ($list.length > 0 &amp;&amp; !(Instanda.Variables.Sector_CHOICE == &quot; , &quot;'&quot; , &quot;Informatique&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.Turnover_NUM &lt;= 95000 &amp;&amp; Instanda.Variables.InsuranceType_CHOICE == &quot; , &quot;'&quot; , &quot;Votre activité professionnelle&quot; , &quot;'&quot; , &quot;)){
        var $covers = $(&quot; , &quot;'&quot; , &quot;.cover-wrapper&quot; , &quot;'&quot; , &quot;, $list);
        var lastIndex = $covers.length - 1;

	//Commented by (DanielPeresFR ) - APFD-1233 and APFD-1234

       /* var removeLastCover =
          (Instanda.Variables.Sector_CHOICE === &quot; , &quot;'&quot; , &quot;Véhicule de Tourisme avec Chauffeur&quot; , &quot;'&quot; , &quot;) ||
          (Instanda.Variables.Sector_CHOICE === &quot; , &quot;'&quot; , &quot;Artisans, Commerçants et e-Commerçants de détail&quot; , &quot;'&quot; , &quot;
            &amp;&amp; Instanda.Variables.InsuranceType_CHOICE === &quot; , &quot;'&quot; , &quot;Votre activité professionnelle&quot; , &quot;'&quot; , &quot;);

        if (removeLastCover) {
          $covers.last().remove();
          var lastIndex = $covers.length - 2;
        }*/

        var $lastCover = $covers[lastIndex];
        $(&quot; , &quot;'&quot; , &quot;.cover-content > div&quot; , &quot;'&quot; , &quot;, $lastCover).each(function(itemIndex, item) {
          $covers.slice(0, lastIndex).each(function(coverIndex, cover) {
            const $coverItems = $(&quot; , &quot;'&quot; , &quot;.cover-content > div&quot; , &quot;'&quot; , &quot;, cover)
            const length = $coverItems.length;
            if (itemIndex >= length) {
              $(&quot; , &quot;'&quot; , &quot;.cover-content input&quot; , &quot;'&quot; , &quot;, cover).before($(item).clone().addClass(&quot; , &quot;'&quot; , &quot;grey-out&quot; , &quot;'&quot; , &quot;));
            }
          });
        });
      }
    }

    var toggleSelectedClass = function() {
      const toggleClass = function(radio, box) {
        var checked = $(radio).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;);
        if (checked) {
          $(box).addClass(&quot; , &quot;'&quot; , &quot;instanda-selected&quot; , &quot;'&quot; , &quot;)
          $(boxes).not(box).removeClass(&quot; , &quot;'&quot; , &quot;instanda-selected&quot; , &quot;'&quot; , &quot;);
        } else {
          $(box).removeClass(&quot; , &quot;'&quot; , &quot;instanda-selected&quot; , &quot;'&quot; , &quot;);
        }
      }
      const boxes = $(&quot; , &quot;'&quot; , &quot;.cover-wrapper > .box&quot; , &quot;'&quot; , &quot;);

      boxes
        .each(function(index, box) {
          var $radio = $(&quot; , &quot;'&quot; , &quot;input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;, box)
          toggleClass($radio, box, boxes);
        })
        .once(&quot; , &quot;'&quot; , &quot;coverWrapperClick&quot; , &quot;'&quot; , &quot;)
        .on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(event) {
          const box = event.currentTarget;
          event.stopPropagation();
          var $radio = $(&quot; , &quot;'&quot; , &quot;input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;, box);
          $radio.prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;, true).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
          toggleClass($radio, box, boxes);
        });
    }

    var buildSwiper = function() {
      $(&quot; , &quot;'&quot; , &quot;#instanda-quote-content&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;buildSwiper&quot; , &quot;'&quot; , &quot;).each(function(elem) {
        var $slides = $(&quot; , &quot;'&quot; , &quot;.cover-list > .cover-wrapper&quot; , &quot;'&quot; , &quot;, elem).addClass(&quot; , &quot;'&quot; , &quot;swiper-slide&quot; , &quot;'&quot; , &quot;);
        var $wrapper = $slides.parent().addClass(&quot; , &quot;'&quot; , &quot;swiper-wrapper&quot; , &quot;'&quot; , &quot;).wrap(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;my-swiper-container&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);
        $wrapper.parent().append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;swiper-button-prev&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);
        $wrapper.parent().append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;swiper-button-next&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);

        var swiperOption = {
          watchOverflow: true,
          slidesPerView: Instanda.utils.isMobile() ? 1 : $slides.length,
          navigation: {
            nextEl: &quot; , &quot;'&quot; , &quot;.swiper-button-next&quot; , &quot;'&quot; , &quot;,
            prevEl: &quot; , &quot;'&quot; , &quot;.swiper-button-prev&quot; , &quot;'&quot; , &quot;,
          }
        }
        var swiper = new Swiper(&quot; , &quot;'&quot; , &quot;.my-swiper-container&quot; , &quot;'&quot; , &quot;, swiperOption);
        swiper.slideTo(1);
      });
    }

    var buildToolTips = function() {
      $(&quot; , &quot;'&quot; , &quot;.help-text&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;overlay-popup&quot; , &quot;'&quot; , &quot;).each(function(index, item) {
        var $icon = $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, item);
        var $content = $(&quot; , &quot;'&quot; , &quot;&lt;div>&quot; , &quot;'&quot; , &quot; + $icon.data(&quot; , &quot;'&quot; , &quot;content&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot;);

        if ($icon.length > 0 &amp;&amp; $content.length > 0) {
          tippy($icon[0], {
            content: $content[0]
          });
        }
      });
    };

    var construct = function() {
      moveRadioBtn();
      syncCoverItems();
      toggleSelectedClass();
      buildSwiper();
      buildToolTips();
      disableContinueBtn();
    }

    $(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function() {
      setTimeout(function() {
        toggleSelectedClass();
      }, 0);
    });
    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Pre Quote Questions (Page 1).
   */
  (function() {
    // do nothing if it is not pre quote questions page 1.
    if ((Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;prequotequestions&quot; , &quot;'&quot; , &quot;
      &amp;&amp; Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;prequotequestionscontinue&quot; , &quot;'&quot; , &quot;)
      || Instanda.Variables.PageNumber != 1) return;

    var createMultiStepForm = function() {
      if (Instanda.utils.isMobile()) {
        $(&quot; , &quot;'&quot; , &quot;#innerBody form&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;multiStepForm&quot; , &quot;'&quot; , &quot;).each(function() {
          var multiStep = new Instanda.multiStepForm();
          multiStep.addForm(&quot; , &quot;'&quot; , &quot;.instanda-pre-quote-questions > .instanda-questionList > .instanda-well:nth-child(2)&quot; , &quot;'&quot; , &quot;);
          multiStep.addForm(&quot; , &quot;'&quot; , &quot;.instanda-pre-quote-questions > .instanda-questionList > .instanda-well:nth-child(3)&quot; , &quot;'&quot; , &quot;);
          multiStep.setContinueBtn(&quot; , &quot;'&quot; , &quot;[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;);
          multiStep.setBackBtn(&quot; , &quot;'&quot; , &quot;#backButton&quot; , &quot;'&quot; , &quot;);
          multiStep.setNextBtn(&quot; , &quot;'&quot; , &quot;&lt;a class=&quot;btn btn-primary&quot;>Étape suivante&lt;/a>&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;insertAfter&quot; , &quot;'&quot; , &quot;);
          multiStep.setPreviousBtn(&quot; , &quot;'&quot; , &quot;&lt;a class=&quot;btn btn-primary&quot;>Étape précédente&lt;/a>&quot; , &quot;'&quot; , &quot;, $(multiStep.continueBtn.parent(&quot; , &quot;'&quot; , &quot;.pull-right&quot; , &quot;'&quot; , &quot;)).siblings(&quot; , &quot;'&quot; , &quot;.pull-left&quot; , &quot;'&quot; , &quot;), &quot; , &quot;'&quot; , &quot;appendTo&quot; , &quot;'&quot; , &quot;);
          multiStep.start();
        });
      }
    }

    // pre-fill address
    var autofillAddress = function() {
      setTimeout(function() {
        if (Instanda.Variables.RiskAddress_YN === &quot;Confirmed&quot;) {
          $(&quot; , &quot;'&quot; , &quot;#CompanyName_TXT&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.EnterpriseName_TXT);
          $(&quot; , &quot;'&quot; , &quot;#CompanyName_TXT--mdc&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.EnterpriseName_TXT).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
          $(&quot; , &quot;'&quot; , &quot;#CompanyAddressLine_TXT&quot; , &quot;'&quot; , &quot;).val([Instanda.Variables.AddressLine1,Instanda.Variables.AddressLine2].join(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;).trim());
          $(&quot; , &quot;'&quot; , &quot;#CompanyAddressLine_TXT--mdc&quot; , &quot;'&quot; , &quot;).val([Instanda.Variables.AddressLine1,Instanda.Variables.AddressLine2].join(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;).trim()).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
          $(&quot; , &quot;'&quot; , &quot;#CompanyPostcode_TXT&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.Postcode.toString().padStart(5, &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;));
          $(&quot; , &quot;'&quot; , &quot;#CompanyPostcode_TXT--mdc&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.Postcode.toString().padStart(5, &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;)).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
          $(&quot; , &quot;'&quot; , &quot;#CompanyCity_TXT&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.City);
          $(&quot; , &quot;'&quot; , &quot;#CompanyCity_TXT--mdc&quot; , &quot;'&quot; , &quot;).val(Instanda.Variables.City).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
        }
      }, 1000);
    }

    // set autocomplete attributes
    var addAutoComplete = function() {
      $(&quot; , &quot;'&quot; , &quot;#FirstName&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;given-name&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#LastName&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;family-name&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#Email_TXT&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#TelephoneNumber_NUM&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tel&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#CompanyName_TXT&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;organization&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#CompanyAddressLine_TXT&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;street-address&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#CompanyPostcode_TXT&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;autocomplete&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;postal-code&quot; , &quot;'&quot; , &quot;);
    }

    var construct = function() {
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;prequotequestions&quot; , &quot;'&quot; , &quot;) {
        createMultiStepForm();
      }
      addAutoComplete();
    }

    $(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, autofillAddress);

    $(document)
      .ready(function() {
        construct();
      })
      .ajaxComplete(construct);
  })();


  /**
   * Pre Quote Questions (Page 2).
   */
  (function() {
    // do nothing if it is not pre quote questions page 2.
    if (Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;prequotequestions&quot; , &quot;'&quot; , &quot; || Instanda.Variables.PageNumber != 2) return;

    // remove bootstrap grid class for ease of style
    var removeColClass = function() {
      $(&quot; , &quot;'&quot; , &quot;.instanda-main-content .questionList&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;[class^=&quot;col-&quot;], [class*=&quot; col-&quot;]&quot; , &quot;'&quot; , &quot;).each(function() {
        var cls = this.getAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;).replace(/col-\S*/g, &quot;&quot;).trim().replace(/\s+/g, &quot; &quot;);
        this.setAttribute(&quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot;, cls);
      });
    }

    // move tool tip icon
    var moveIcon = function() {
      $(&quot; , &quot;'&quot; , &quot;.instanda-main-content .questionList&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.instanda-question-help-col&quot; , &quot;'&quot; , &quot;).once(&quot; , &quot;'&quot; , &quot;moveIcon&quot; , &quot;'&quot; , &quot;).each(function() {
        var $iconContainer = $(this);
        $iconContainer.parent().parent().append($iconContainer);
      });
    }

    var construct = function() {
      removeColClass();
      moveIcon();
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Pre Quote Questions (Page 3).
   */
  (function() {
    // do nothing if it is not pre quote questions page 3.
    if (Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;prequotequestions&quot; , &quot;'&quot; , &quot; || Instanda.Variables.PageNumber != 3) return;

    // set min policy start date
    var setMinStartDate = function() {
      var $continueBtn = $(&quot; , &quot;'&quot; , &quot;[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;);
      var $startDateInput = $(&quot; , &quot;'&quot; , &quot;#PolicyStartDate_DATE&quot; , &quot;'&quot; , &quot;);

      $startDateInput.once(&quot; , &quot;'&quot; , &quot;setMinStartDate&quot; , &quot;'&quot; , &quot;).each(function(elem) {
        var DatePicker = $startDateInput.parent(&quot; , &quot;'&quot; , &quot;.instanda-question-date&quot; , &quot;'&quot; , &quot;).data(&quot;DateTimePicker&quot;);

        if (DatePicker) {
          // bootstrap-datepicker
          DatePicker.minDate(moment().startOf(&quot; , &quot;'&quot; , &quot;d&quot; , &quot;'&quot; , &quot;));
        } else {
          // native
          $startDateInput[0].min = moment().startOf(&quot; , &quot;'&quot; , &quot;d&quot; , &quot;'&quot; , &quot;).format(&quot; , &quot;'&quot; , &quot;YYYY-MM-DD&quot; , &quot;'&quot; , &quot;);
        }
        $startDateInput.on(&quot; , &quot;'&quot; , &quot;focus&quot; , &quot;'&quot; , &quot;, function() {
          $continueBtn.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
        }).on(&quot; , &quot;'&quot; , &quot;blur&quot; , &quot;'&quot; , &quot;, function() {
          $continueBtn.prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
        });
      });
    }

    var construct = function() {
      setMinStartDate()
    }

    $(document)
      .ready(construct)
      .ajaxComplete(construct);
  })();


  /**
   * Move &quot;instanda-well&quot; help text banner
   */
  (function() {
    var moved = false;

    var _moveBanner = function(target) {
      const $banner = $(&quot; , &quot;'&quot; , &quot;.instanda-main-content&quot; , &quot;'&quot; , &quot;).find(target);
      if (!moved &amp;&amp; $banner.length > 0) {
        $banner.prependTo(&quot; , &quot;'&quot; , &quot;.instanda-main-content-container&quot; , &quot;'&quot; , &quot;);
        moved = true;
      } else {
        $banner.remove();
      }
    }

    var moveBanner = function() {
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;prequotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == 1) {
        _moveBanner(&quot; , &quot;'&quot; , &quot;#question118956&quot; , &quot;'&quot; , &quot;);
      }
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;prequotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == 2) {
        _moveBanner(&quot; , &quot;'&quot; , &quot;#question120226&quot; , &quot;'&quot; , &quot;);
      }
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;prequotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == 3) {
        _moveBanner(&quot; , &quot;'&quot; , &quot;#question120292&quot; , &quot;'&quot; , &quot;);
      }
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quickquote&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == 1) {
        _moveBanner(&quot; , &quot;'&quot; , &quot;#instanda-quote-content .questionList&quot; , &quot;'&quot; , &quot;);
      }
      if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;postquotequestions&quot; , &quot;'&quot; , &quot;) {
        _moveBanner(&quot; , &quot;'&quot; , &quot;#question120291&quot; , &quot;'&quot; , &quot;);
        _moveBanner(&quot; , &quot;'&quot; , &quot;#question119073&quot; , &quot;'&quot; , &quot;);
        _moveBanner(&quot; , &quot;'&quot; , &quot;#question120358&quot; , &quot;'&quot; , &quot;);
      }
    }

    $(document)
      .ready(moveBanner)
      .ajaxComplete(moveBanner);
  })();


  /////////////////////////////////////////////////////////////////////////////// CHECKBOXES
  (function() {
    //------Add class to selected options on questions and covers on all pages
    function add_class_to_selected_option_on_question_and_quote_pages_for_checkboxes() {
      $(&quot; , &quot;'&quot; , &quot;label input[type=&quot;checkbox&quot;]&quot; , &quot;'&quot; , &quot;).each(function(){
        if ($(this).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
          $(this).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;ticked&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;unticked&quot; , &quot;'&quot; , &quot;);
        } else {
          $(this).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;unticked&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;ticked&quot; , &quot;'&quot; , &quot;);
        }
      });
    }

    //-----When clicked, on question pages
    $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click change&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;label input[type=&quot;checkbox&quot;]&quot; , &quot;'&quot; , &quot;, function(){
      if ($(this).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
        $(this).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;ticked&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;unticked&quot; , &quot;'&quot; , &quot;);
      } else {
        $(this).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;unticked&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;ticked&quot; , &quot;'&quot; , &quot;);
      }
    } );

    //-----Add class to selected options on questions and covers on all pages
    function add_class_to_disabled_option_on_question_and_quote_pages_for_checkboxes() {
      $(&quot; , &quot;'&quot; , &quot;label input[type=&quot;checkbox&quot;][disabled=&quot;disabled&quot;]&quot; , &quot;'&quot; , &quot;).each(function(){
        $(this).parent(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;);
      });
    }

    //-----Run----------
    $(document).ready(function () {
      add_class_to_selected_option_on_question_and_quote_pages_for_checkboxes();
      add_class_to_disabled_option_on_question_and_quote_pages_for_checkboxes();
    });

    $(document).ajaxSuccess(function() {
      add_class_to_selected_option_on_question_and_quote_pages_for_checkboxes();
      add_class_to_disabled_option_on_question_and_quote_pages_for_checkboxes();
    });
  })();


  /**
   * Customer Portal.
   */
  (function() {
    setTimeout(function() {
      // open tab if there is hash value on url
      if (document.location.hash.length > 0) {
        $(&quot; , &quot;'&quot; , &quot;.nav-item a[href=&quot;&quot; , &quot;'&quot; , &quot;+document.location.hash+&quot; , &quot;'&quot; , &quot;&quot;]&quot; , &quot;'&quot; , &quot;).tab(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
      }

      // change navigate history when user switch tabs.
      $(&quot; , &quot;'&quot; , &quot;.nav-item a&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;shown.bs.tab&quot; , &quot;'&quot; , &quot;, function(e) {
        if (history.pushState) {
          window.history.pushState(null, null, e.target.hash);
        }
      });
    }, 0);

    if (Instanda.Variables.PageName != &quot; , &quot;'&quot; , &quot;customerhomepage&quot; , &quot;'&quot; , &quot;) return;

    $(&quot; , &quot;'&quot; , &quot;.accordion-doc [data-toggle=&quot;collapse&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(event) {
      var collapse = $(event.target).next().collapse(&quot; , &quot;'&quot; , &quot;toggle&quot; , &quot;'&quot; , &quot;);
      $(event.target).attr(&quot; , &quot;'&quot; , &quot;aria-expanded&quot; , &quot;'&quot; , &quot;, collapse.attr(&quot; , &quot;'&quot; , &quot;aria-expanded&quot; , &quot;'&quot; , &quot;));
    });
  })();


  /**
   * Customer Portal Edit Details Page.
   *
   * Insert content to Edit Customer Details page
   */
  (function() {
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customerdetailschangepassword&quot; , &quot;'&quot; , &quot;){
      $(&quot; , &quot;'&quot; , &quot;.page-customerdetailschangepassword .instanda-main-content&quot; , &quot;'&quot; , &quot;).prepend(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;container&quot;>&lt;h3>Modification de votre mot de passe&lt;/h3>&lt;p style=&quot;margin-bottom: 40px;&quot;>Merci de renseigner ci-dessous votre nouveau mot de passe.&lt;/p>&lt;/div>&quot; , &quot;'&quot; , &quot;);
    }
  })();


  /**
   * Customer Portal Password Reset Sent.
   *
   * Add &quot;alert-success&quot; class to password reset succeeded message
   */
  (function() {
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customersendpasswordresetlink&quot; , &quot;'&quot; , &quot;){
      $(&quot; , &quot;'&quot; , &quot;#instanda-cp-reset-link-form .section_content&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;alert alert-success&quot; , &quot;'&quot; , &quot;);
    }
  })();


  /////////////////////////////////////////////////////////////////////////////// Translation
  (function() {
    // Post quote questions page 1.
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;postquotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber === 1) {
      $(&quot; , &quot;'&quot; , &quot;button[name=&quot;continueButton&quot;]&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;Souscrire en ligne&quot; , &quot;'&quot; , &quot;);
    }

    // Customer Portal Login Page
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customerlogin&quot; , &quot;'&quot; , &quot;) {
      $(&quot; , &quot;'&quot; , &quot;button[type=&quot;submit&quot;]&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Me connecter&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#Password&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Votre mot de passe&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;input[placeholder=&quot;email address&quot;]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Votre email&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#instanda-cp-forgot-link&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Je ne retrouve plus mon mot de passe&quot; , &quot;'&quot; , &quot;);
    }

    // Customer Portal Forget Password and Send Password Reset Pages
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customersendpasswordresetlink&quot; , &quot;'&quot; , &quot;) {
      $(&quot; , &quot;'&quot; , &quot;#instanda-cp-user-name&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Identifiant (email)&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;button[type=&quot;submit&quot;]&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Recevoir mon mot de passe par email&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#instanda-cp-reset-link-form a&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;Renvoyer&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;data-loading-text&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Renvoyer&quot; , &quot;'&quot; , &quot;);
    }

    // Customer Portal Input and Confirm New Password Page
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customerforgotpassword&quot; , &quot;'&quot; , &quot;) {
      $(&quot; , &quot;'&quot; , &quot;#NewPassword&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Votre nouveau mot de passe&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#ConfirmPassword&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Confirmez votre nouveau mot de passe&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#instanda-cp-charge-password&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Changer mon mot de passe&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;data-loading-text&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Changer mon mot de passe&quot; , &quot;'&quot; , &quot;);
    }

    // Customer Portal Your Account Details Page
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customerdetails&quot; , &quot;'&quot; , &quot;) {
      $(&quot; , &quot;'&quot; , &quot;label.control-label:contains(&quot;Username&quot;)&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Identifiant (email)&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;label.control-label:contains(&quot;Password&quot;)&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Mot de passe&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#password-fake-label + a&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;Modifier&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;.instanda-update-customer-details-button&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Mettre à jour&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#savemsg strong&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;Vos coordonnées ont été mises à jour&quot; , &quot;'&quot; , &quot;);
    }

    // Customer Portal Change Password Page (enter from Your Account Details Page)
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customerdetailschangepassword&quot; , &quot;'&quot; , &quot;) {
      $(&quot; , &quot;'&quot; , &quot;label.control-label:contains(&quot;Old&quot;)&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Votre mot de passe&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;label.control-label:contains(&quot;Confirm&quot;)&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Confirmez votre nouveau mot de passe&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;label.control-label:contains(&quot;New&quot;)&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Votre nouveau mot de passe&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;#instanda-save-password-button&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Mettre à jour&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;data-loading-text&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;enregistrer&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;form a:contains(&quot;Cancel&quot;)&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;Annuler&quot; , &quot;'&quot; , &quot;);
    }

    // Registration
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;existingpolicyholdersendregistrationlink&quot; , &quot;'&quot; , &quot; || Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;publicuatuserlogin&quot; , &quot;'&quot; , &quot; || Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;customerregister&quot; , &quot;'&quot; , &quot;) {
      $(&quot; , &quot;'&quot; , &quot;#instanda-cp-user-name&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;placeholder&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Identifiant (email)&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;button[type=&quot;submit&quot;]&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Continuer&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;data-loading-text&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;Continuer&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;label.control-label:contains(&quot;E-mail address&quot;)&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Votre email&quot; , &quot;'&quot; , &quot;);
      $(&quot; , &quot;'&quot; , &quot;label.control-label:contains(&quot;Password&quot;)&quot; , &quot;'&quot; , &quot;).text(&quot; , &quot;'&quot; , &quot;Votre mot de passe&quot; , &quot;'&quot; , &quot;);
    }

    // Registration error message contains dynamic data, email address
    $(&quot; , &quot;'&quot; , &quot;.validation-summary-errors li&quot; , &quot;'&quot; , &quot;).each(function(index, elem) {
      const text = $(elem).text();
      const regex = /E-mail address &quot; , &quot;'&quot; , &quot;(.*)&quot; , &quot;'&quot; , &quot; has already been registered./;
      const matches = text.match(regex);

      if (matches) {
        $(elem).text(&quot; , &quot;'&quot; , &quot;Un compte existe déjà pour cette adresse e-mail. Merci de cliquer sur \&quot; , &quot;'&quot; , &quot;Me connecter\&quot; , &quot;'&quot; , &quot; pour accéder à votre Espace client.&quot; , &quot;'&quot; , &quot;);
      }
    });

    const translations = {
      &quot; , &quot;'&quot; , &quot;Log Out&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Déconnexion&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Log in&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Me connecter&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Your Account&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Vos coordonnées&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Continue&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Étape suivante&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Email address is required&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Adresse e-mail est nécessaire&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Please use a valid email address.&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Veuillez utiliser une adresse mail valide&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;The Password field is required.&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Le champ Mot de passe est obligatoire&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Password is required&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Mot de passe requis&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;New password is required&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Un nouveau mot de passe est requis&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Confirm password is required&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Confirmer le mot de passe est requis&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;City&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Ville&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Password must be at least 8 characters.&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Le mot de passe doit contenir au moins 8 caractères&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Password must contain at least 1 numbers.&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Le mot de passe doit contenir au moins 1 chiffre&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Passwords must include at least 1 special characters (!#@ etc)&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Le mot de passe doit contenir au moins 1 caractère spécial (!#@ etc)&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Invalid password&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Mot de passe incorrect&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Passwords do not match&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Les mots de passe ne correspondent pas&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Password must be between 6 and 99 characters&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Le mot de passe doit comporter entre 6 et 99 caractères&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;User is already registered&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Un compte lié à cette adresse email existe déjà&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;If that email address is in our database, we will send you an email to reset your password.&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Les instructions pour changer / recouvrer votre mot de passe vous ont été envoyés par email.&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Policy start date cannot be in the past&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;La date d\&quot; , &quot;'&quot; , &quot;effet du contrat ne peut pas être dans le passé. Pour finaliser votre contrat, merci de choisir une nouvelle date d\&quot; , &quot;'&quot; , &quot;effet. (+ indiquer qu\&quot; , &quot;'&quot; , &quot;il faut cliquer sur \&quot; , &quot;'&quot; , &quot;Précédent\&quot; , &quot;'&quot; , &quot;?&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;Authentication failed.&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Échec de l\&quot; , &quot;'&quot; , &quot;authentification&quot; , &quot;'&quot; , &quot;,
      &quot; , &quot;'&quot; , &quot;You are logged in with a different email address, you need to logout first to proceed with this quote&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Vous êtes connecté avec une adresse e-mail différente, vous devez d\’abord vous déconnecter pour continuer avec ce devis&quot; , &quot;'&quot; , &quot;,
&quot; , &quot;'&quot; , &quot;Logout and return to quote&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;Déconnecter pour continuer avec ce devis&quot; , &quot;'&quot; , &quot;,
    }

    // @see https://stackoverflow.com/a/42041097/1146907
    const innerBody = document.getElementById(&quot; , &quot;'&quot; , &quot;innerBody&quot; , &quot;'&quot; , &quot;);
    const walker = document.createTreeWalker(innerBody, NodeFilter.SHOW_TEXT, null, false);
    while (walker.nextNode()) {
      for (const source in translations) {
        if (walker.currentNode.nodeValue.trim() == source) {
          walker.currentNode.nodeValue = translations[source];
        }
      }
    }

    const messageFields = document.querySelectorAll(&quot; , &quot;'&quot; , &quot;[data-valmsg-for]&quot; , &quot;'&quot; , &quot;);
    const callback = function(mutationList, observer) {
      for (let mutation of mutationList) {
        for (const source in translations) {
          if (mutation.addedNodes &amp;&amp; mutation.addedNodes.length > 0 &amp;&amp; mutation.addedNodes[0].innerText.trim() == source) {
            mutation.addedNodes[0].innerText = translations[source];
          }
        }
      }
    }
    const observer = new MutationObserver(callback);
    const config = { childList: true };
    messageFields.forEach(function(field) {
      observer.observe(field, config);
    });

  })();


// HXF-3 - Postcode look up
const lookUpPostcode = function() {
  var _PostcodeFields = [];
  _PostcodeFields.push({
    postcode: &quot;input#Postcode118934&quot;,
    city: &quot;input#City&quot;,
    container: &quot;.form-group&quot;,
    wrapper: &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;instanda-question-item form-group container postcode-look-up&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;
  }); // Page 3
  _PostcodeFields.push({
    postcode: &quot;input#Postcode_FormB_NUM&quot;,
    city: &quot;input#City_FormB&quot;,
    container: &quot;.questionItem&quot;,
    wrapper: &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;questionItem instanda-question-item form-group container postcode-look-up&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;
  }); // Page 4
  _PostcodeFields.push({
    postcode: &quot;input#CompanyPostcode_TXT&quot;,
    city: &quot;input#CompanyCity_TXT&quot;,
    container: &quot;.questionItem&quot;,
    wrapper: &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;questionItem instanda-question-item form-group container postcode-look-up&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;
  }); // Page 5

  $.each(_PostcodeFields, function(key, value){
    $(value.city).prop(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;, true).trigger(&quot;mdc:rebuild&quot;);

    window.setTimeout(function () {
      if ($(value.postcode).length > 0) {
        $(value.postcode).on(&quot;change&quot;, function () {
          if (typeof $(value.postcode).val() !== &quot;undefined&quot; &amp;&amp; $(value.postcode).val().trim() !== &quot;&quot;) {
            $.get(&quot;https://geo.api.gouv.fr/communes&quot;, { codePostal: $(value.postcode).val().replace(/ /g, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) }).done(
              function (data) {
                if ($(&quot;select.gc-fr-postale-code&quot;).length > 0) {
                  $(&quot;select.gc-fr-postale-code&quot;).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-item&quot; , &quot;'&quot; , &quot;).remove();
                }
                $(value.city+&quot; , &quot;'&quot; , &quot;--mdc&quot; , &quot;'&quot; , &quot;).focus();
                $(value.city).val(&quot;&quot;).trigger(&quot;change&quot;);
                if (data.length > 0) {
                  var _postal = data;
                  if (typeof _postal !== &quot;undefined&quot; &amp;&amp; _postal !== null) {
                    if(_postal.length === 1){
                      $(value.city).val(_postal[0].nom).trigger(&quot;change&quot;);
                    } else {
                      var _ddl = $(&quot;&lt;select>&quot;)
                        .addClass(&quot;gc-fr-postale-code&quot;)
                        .on(&quot;change&quot;, function(){
                          $(value.city).val($(this).val()).trigger(&quot;mdc:rebuild&quot;);
                        });
                      $.each(_postal, function(key, value){
                        $(_ddl).append(
                          $(&quot;&lt;option/>&quot;)
                          .attr({ value: value.nom })
                          .text(value.nom)
                        );
                      });
                      $(value.city).val(_ddl.val()).trigger(&quot;mdc:rebuild&quot;);
                      var _ddlContainer = $(value.wrapper).append(_ddl);
                      $(value.city+&quot; , &quot;'&quot; , &quot;--mdc&quot; , &quot;'&quot; , &quot;).closest(value.container).before(_ddlContainer);
                    }
                  }
                } else {
                  //$(value.city).focus();
                  $(value.city).val(&quot;&quot;); //.trigger(&quot;change&quot;);
                }
              }
            );
          }
        });
      }
    });
  },2000);
}

$(document).ready(lookUpPostcode);



  /* Quote Display */
  (function () {
    // send email and continue in one click
    if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot; &amp;&amp; (Instanda.Variables.PageNumber == 1 || Instanda.Variables.PageNumber == 5 )) {
      const fakeBtn = $(&quot; , &quot;'&quot; , &quot;.sidebar-cta .btn-fake&quot; , &quot;'&quot; , &quot;);
      const nextBtn = $(&quot; , &quot;'&quot; , &quot;.sidebar-cta .instanda_nav_link&quot; , &quot;'&quot; , &quot;).hide();
      const nextBtnBelow = $(&quot; , &quot;'&quot; , &quot;#continueButton&quot; , &quot;'&quot; , &quot;).hide();
      const fakeBtnBelow = $(&quot; , &quot;'&quot; , &quot;&lt;button type=&quot;submit&quot; name=&quot;continueButton&quot; class=&quot;instanda-button instanda-quote-continue-button btn btn-primary&quot;>Recevoir mon devis par email &lt;i class=&quot;fa fa-caret-right&quot;>&lt;/i>&lt;/button>&quot; , &quot;'&quot; , &quot;).insertBefore(nextBtnBelow); 
      const emailLink = $(&quot; , &quot;'&quot; , &quot;#emailQuoteLinkLink&quot; , &quot;'&quot; , &quot;).hide();

      fakeBtnBelow.on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function () {
        emailLink.trigger(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;);
        /* commented by RitaHiscoxFrance 13/04/2022 INST #20373 - begin {
        setTimeout(function () {
          nextBtnBelow.trigger(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;);
        }, 200);
        \* end } - commented by RitaHiscoxFrance 13/04/2022 INST #20373 */
      });

      fakeBtn.on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function () {
        emailLink.trigger(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;);
        setTimeout(function () {
          nextBtn.trigger(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;);
        }, 200);
      });
    }
  })();



  // swap address fields order
  (function() {
    const addressContainer = $(&quot; , &quot;'&quot; , &quot;#question118934&quot; , &quot;'&quot; , &quot;);
    if (addressContainer.length > 0) {
      const postCode = $(&quot; , &quot;'&quot; , &quot;.instanda-address-postcode&quot; , &quot;'&quot; , &quot;, addressContainer).closest(&quot; , &quot;'&quot; , &quot;.instanda-question-item&quot; , &quot;'&quot; , &quot;);
      const city = $(&quot; , &quot;'&quot; , &quot;.instanda-address-city&quot; , &quot;'&quot; , &quot;, addressContainer).closest(&quot; , &quot;'&quot; , &quot;.form-group&quot; , &quot;'&quot; , &quot;);
      if (postCode.length > 0 &amp;&amp; city.length > 0) {
        postCode.insertBefore(city);
      }
    }
  })();


  /**
   * Post-Quote Questions (Page 1).
   */
  (function() {
    // set download document URL
    if (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;postquotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;) {
      const matches = document.location.search.match(/[?|&amp;]quoteRef=(.*?)[$|&amp;]/);
      const quoteRef = matches[1];

      if (Instanda.Variables.InsuranceType_CHOICE === &quot; , &quot;'&quot; , &quot;Votre activité professionnelle&quot; , &quot;'&quot; , &quot;) {
        const docLink = &quot; , &quot;'&quot; , &quot;/Public/DownloadStoredPdfBuilder?packageId=&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;12462&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&amp;pdfBuilderId=&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;305624&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&amp;quoteRef=&quot; , &quot;'&quot; , &quot; + quoteRef;
        const link = $(&quot; , &quot;'&quot; , &quot;.download-link&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, docLink);
      }

      if (Instanda.Variables.InsuranceType_CHOICE === &quot; , &quot;'&quot; , &quot;Vos locaux&quot; , &quot;'&quot; , &quot;) {
        const docLink = &quot; , &quot;'&quot; , &quot;/Public/DownloadStoredPdfBuilder?packageId=&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;12462&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&amp;pdfBuilderId=&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;305637&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&amp;quoteRef=&quot; , &quot;'&quot; , &quot; + quoteRef;
        const link = $(&quot; , &quot;'&quot; , &quot;.download-link&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, docLink);
      }

      if (Instanda.Variables.InsuranceType_CHOICE === &quot; , &quot;'&quot; , &quot;Votre activité &amp; vos locaux&quot; , &quot;'&quot; , &quot;) {
        const docLink = &quot; , &quot;'&quot; , &quot;/Public/DownloadStoredPdfBuilder?packageId=&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;12462&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&amp;pdfBuilderId=&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;305652&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&amp;quoteRef=&quot; , &quot;'&quot; , &quot; + quoteRef;
        const link = $(&quot; , &quot;'&quot; , &quot;.download-link&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, docLink);
      }
    }
  })();


  /**
   * Post-Quote Questions (Page 2).
   */
  (function() {
    // set download document URL
    if ((Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;postquotequestions&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;) || (Instanda.Variables.PageName === &quot; , &quot;'&quot; , &quot;postquotequestionscontinue&quot; , &quot;'&quot; , &quot; &amp;&amp; Instanda.Variables.PageNumber == &quot; , &quot;'&quot; , &quot;2&quot; , &quot;'&quot; , &quot;)) {
      const modifyLink = function() {
        const $modifyLink = $(&quot; , &quot;'&quot; , &quot;.policy-start-date [data-placeholder=&quot;~publicPreQuoteLink&quot;]&quot; , &quot;'&quot; , &quot;);
        const href = $modifyLink.prop(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;);
        const url = new URL(href);
        url.searchParams.set(&quot; , &quot;'&quot; , &quot;pageNumber&quot; , &quot;'&quot; , &quot;, 3);
        url.searchParams.set(&quot; , &quot;'&quot; , &quot;goBackwards&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;True&quot; , &quot;'&quot; , &quot;);
        $modifyLink.prop(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, url.toString());
      }
      $(document).ready(modifyLink);

      const fillDummyAddress = function() {
        setTimeout(function () {
            $(&quot; , &quot;'&quot; , &quot;#InsuredAddressLine1_TXT&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#InsuredAddressLine2_TXT&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;Street&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;#InsuredAddressCity_TXT&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;City&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;input[name=&quot;InsuredPostcode_TXT&quot;]&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;12345&quot; , &quot;'&quot; , &quot;).trigger(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
        }, 0);
      }
      $(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, fillDummyAddress);
    }
  })();


  /* Phone widget */
  (function() {
    $(&quot; , &quot;'&quot; , &quot;.tel-icon-wannaspeak&quot; , &quot;'&quot; , &quot;).click(function() {
      wsCall();
    });
  })();


// START
// HXF-8: [PI Public] Real time class toggling on valid fields
// GC 01/10/2020
$(document).ready(function(){
    // Creates and returns a unique id
    function uuidv4() {
      return &quot; , &quot;'&quot; , &quot;xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx&quot; , &quot;'&quot; , &quot;.replace(/[xy]/g, function(c) {
        var r = Math.random() * 16 | 0, v = c == &quot; , &quot;'&quot; , &quot;x&quot; , &quot;'&quot; , &quot; ? r : (r &amp; 0x3 | 0x8);
        return v.toString(16);
      });
    };
    // Configure our fields
    var _fieldsToValidate = [];
    _fieldsToValidate.push({ &quot;ValidOn&quot;: &quot;range&quot;, &quot;Parent&quot;: &quot;div#question119075&quot;, &quot;Selector&quot;: &quot;input#IBAN_TXT&quot;, &quot;Type&quot;: &quot;Text&quot;, &quot;Mdc&quot;: true, &quot;On&quot;: &quot;change input&quot;, &quot;Min&quot;: 27, &quot;Max&quot;: 27, &quot;Css&quot;: &quot;realtime-is-valid&quot; });
    _fieldsToValidate.push({ &quot;ValidOn&quot;: &quot;other&quot;, &quot;Parent&quot;: &quot;div#question119076&quot;, &quot;Selector&quot;: &quot;input#BIC_TXT&quot;, &quot;Type&quot;: &quot;Text&quot;, &quot;Mdc&quot;: true, &quot;On&quot;: &quot;change input&quot;, &quot;Min&quot;: 8, &quot;Max&quot;: 11, &quot;Css&quot;: &quot;realtime-is-valid&quot; });
    _fieldsToValidate.push({ &quot;ValidOn&quot;: &quot;range&quot;, &quot;Parent&quot;: &quot;div#question119077&quot;, &quot;Selector&quot;: &quot;input#Domiciliation_TXT&quot;, &quot;Type&quot;: &quot;Text&quot;, &quot;Mdc&quot;: true, &quot;On&quot;: &quot;change input&quot;, &quot;Min&quot;: 1, &quot;Max&quot;: 999, &quot;Css&quot;: &quot;realtime-is-valid&quot; });
    _fieldsToValidate.push({ &quot;ValidOn&quot;: &quot;range&quot;, &quot;Parent&quot;: &quot;div#question119078&quot;, &quot;Selector&quot;: &quot;input#AccountOwner_TXT&quot;, &quot;Type&quot;: &quot;Text&quot;, &quot;Mdc&quot;: true, &quot;On&quot;: &quot;change input&quot;, &quot;Min&quot;: 1, &quot;Max&quot;: 999, &quot;Css&quot;: &quot;realtime-is-valid&quot; });
    // Initialize our fields
    // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
    // GC 08/10/2020
    // Switch statement added to increase the scope of the validation.
    // BIC is now not a range between 8 and 11 but is either 8 OR 11.
    $.each(_fieldsToValidate, function(key, value){
        if($(value.Parent).length > 0){
            var _guid = uuidv4();
            value.Guid = _guid; // for use when validating
            $(&quot;&lt;span data-span-guid=&quot; , &quot;'&quot; , &quot;&quot; + _guid + &quot;&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;realtime-valid&quot; , &quot;'&quot; , &quot; />&quot;).prependTo(value.Parent);
            // Set tab-index for focus capture
            $(value.Parent).attr({ &quot;data-realtime&quot;: JSON.stringify(value) }).attr({ &quot;tab-index&quot;: key });
            $(value.Parent).on(&quot;click&quot;, function(){
                // When we are focused in the question div bind the behaviour...
                // ...mdc compontent strips it at every focus out...
                var _data = $(this).attr(&quot;data-realtime&quot;); // data attached to question div...
                _data = JSON.parse(_data);
                $(_data.Selector, _data.Parent).data(_data).on(_data.On, function(el){
                    var _d = $(el.target).data(); // data attached to input...
                    var _thisVal = $(_d.Selector, _d.Parent).val();
                    // Check the value matches the input and hide/show css class
                    console.log(&quot;_d ================> &quot;, _d);
                    if(typeof _d !== &quot;undefined&quot; &amp;&amp; typeof _d.ValidOn !== &quot;undefined&quot;){
                        switch(_d.ValidOn.toLowerCase()){
                            case &quot;range&quot;:
                                if (typeof _thisVal !== &quot;undefined&quot; &amp;&amp; _thisVal !== null &amp;&amp; _thisVal.length >= _d.Min &amp;&amp; _thisVal.length &lt;= _d.Max) {
                                    $(_d.Parent).addClass(&quot;realtime-is-valid&quot;);
                                    $(&quot;span[data-span-guid=&quot; , &quot;'&quot; , &quot;&quot; + _d.Guid + &quot;&quot; , &quot;'&quot; , &quot;]&quot;).addClass(_d.Css);
                                } else {
                                    $(_d.Parent).removeClass(&quot;realtime-is-valid&quot;);
                                    $(&quot;span[data-span-guid=&quot; , &quot;'&quot; , &quot;&quot; + _d.Guid + &quot;&quot; , &quot;'&quot; , &quot;]&quot;).removeClass(_d.Css);
                                }
                                break;
                            case &quot;other&quot;:
                                if (typeof _thisVal !== &quot;undefined&quot; &amp;&amp; _thisVal !== null &amp;&amp; (_thisVal.length === _d.Min || _thisVal.length === _d.Max)) {
                                    $(_d.Parent).addClass(&quot;realtime-is-valid&quot;);
                                    $(&quot;span[data-span-guid=&quot; , &quot;'&quot; , &quot;&quot; + _d.Guid + &quot;&quot; , &quot;'&quot; , &quot;]&quot;).addClass(_d.Css);
                                } else {
                                    $(_d.Parent).removeClass(&quot;realtime-is-valid&quot;);
                                    $(&quot;span[data-span-guid=&quot; , &quot;'&quot; , &quot;&quot; + _d.Guid + &quot;&quot; , &quot;'&quot; , &quot;]&quot;).removeClass(_d.Css);
                                }
                                break;
                        }
                    }
                });
            });
            // GC added to trigger the code to check for previous values and autocompletes...
            setTimeout(function(){
                $(value.Parent).trigger(&quot;click&quot;);
                $(value.Selector + (value.Mdc ? &quot;--mdc&quot; : &quot;&quot;), value.Parent).keyup();
            }, 500);
        }
    });
    // GC 08/10/2020
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
});
// END
// HXF-8: [PI Public] Real time class toggling on valid fields
// GC 01/10/2020


  //////////////////////////////////////////////////////////////////////////////// LOADING ANIMATION

  // Wait for window load, custom-made loading effect
  $(document).ready(function() {
    // Animate loader off screen
    $(&quot;.custom-loading-wrapper&quot;).fadeOut(&quot;slow&quot;);
  });

  //wait for page submit, custom-made loading effect.
  document.addEventListener(&quot; , &quot;'&quot; , &quot;submit&quot; , &quot;'&quot; , &quot;, function() {
    ShowWaitingAnimation();
  });


  //////////////////////////////////////////////////////////////////////////////// Honeypot
  $(&quot; , &quot;'&quot; , &quot;#HoneypotHiddenQuestion&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;tabindex&quot; , &quot;'&quot; , &quot;, -1);



//Create the button Valider - Promocode ( Quote adjustment questions )

function insertPromocodeButton() {
	if (Instanda.Variables.PageName == &quot; , &quot;'&quot; , &quot;quote&quot; , &quot;'&quot; , &quot; &amp;&amp; (Instanda.Variables.PageNumber == 1 || Instanda.Variables.PageNumber == 5 )) {
		updateClone = $(&quot;.instanda-quote-update-button&quot;).clone();
		updateClone.attr(&quot;name&quot;, &quot;promocodeBtn&quot;);  
		updateClone.insertAfter(&quot;.promocode .questionItem &quot;);
		updateClone.attr(&quot;class&quot;, &quot;promocode&quot;);  
		var promo_info = $(&quot;.promocode .instanda-question-help-col&quot;);
		promo_info.hide();
		if ($(&quot;.alert.alert-danger.alert-dismissible.show&quot;).length > 0 &amp;&amp; Instanda.Variables.DeclineRule_Promo_PublicSite == &quot;NotDeclineRule&quot;) {
			updateClone.attr(&quot;class&quot;, &quot;promocode error-message&quot;);  
		}
		if ( Instanda.Variables.IsRenewal &amp;&amp; Instanda.Variables.Renewal_Promo == &quot;No&quot;) {
			 Instanda.Variables.PromocodePublic = &quot;&quot;;
		}

	}
}
insertPromocodeButton();


            
            
            
            
                if (!isTouchDevice) {
                    document.write(&quot; , &quot;'&quot; , &quot;\x3Cscript type=&quot;text/javascript&quot; src=&quot;/Theme/js/plugins/bootstrap-miscellaneous/bootstrap-datetimepicker.min.js&quot;>\x3C/script>&quot; , &quot;'&quot; , &quot;);
                }

                // Clear forms fields when then browser back button is used
                $(window).on(&quot;load&quot;, null, function () {

                    var $forms = $(&quot; , &quot;'&quot; , &quot;form&quot; , &quot;'&quot; , &quot;);

                    if ($forms.length !== 0) {

                        $forms.get(0).reset();
                    }
                });
            
            
            
                const clientOriginalShortDatePatternForDatePicker = &quot; , &quot;'&quot; , &quot;dd/mm/yy&quot; , &quot;'&quot; , &quot;;
                const clientOriginalShortDatePattern = &quot; , &quot;'&quot; , &quot;DD/MM/YYYY&quot; , &quot;'&quot; , &quot;;
                datepickerLogic(clientOriginalShortDatePatternForDatePicker, clientOriginalShortDatePattern, isTouchDevice, isResponsiveTheme)
            


    
        
            
                ×
                JavaScript Error
            
            
                ... error text
            
            
                Close
            
        
    

        





            
                

                Confidentialité &amp; Protection des données Conditions d’utilisation et Mentions Légales Politique de cookies

© 2021 Hiscox SA. All rights reserved by Hiscox SA.

                    Site built and hosted using
                    Instanda

    v1.84.0.5

                    insurance software.
                
            


        


    
        function PopUpReadOnlyToaster() {
            toastr.options = {
                &quot;closeButton&quot;: true,
			    &quot;debug&quot;: false,
			    &quot;newestOnTop&quot;: false,
			    &quot;progressBar&quot;: false,
			    &quot;positionClass&quot;: &quot;toast-bottom-full-width&quot;,
			    &quot;preventDuplicates&quot;: true,
			    &quot;onclick&quot;: null,
			    &quot;showDuration&quot;: &quot;300&quot;,
			    &quot;hideDuration&quot;: &quot;1000&quot;,
			    &quot;timeOut&quot;: &quot;5000&quot;,
			    &quot;extendedTimeOut&quot;: &quot;1000&quot;,
			    &quot;showEasing&quot;: &quot;swing&quot;,
			    &quot;hideEasing&quot;: &quot;linear&quot;,
			    &quot;showMethod&quot;: &quot;fadeIn&quot;,
			    &quot;hideMethod&quot;: &quot;fadeOut&quot;,
                &quot;escapeHtml&quot; : true
            }



            toastr[&quot;info&quot;](&quot; , &quot;'&quot; , &quot;This quote is displayed as readonly&quot; , &quot;'&quot; , &quot;);
        }

            
            &quot;use strict&quot;;
			$(function () {
                var common = new Instanda.Common();
                var keeplLockOnPolicyInterval = window.setInterval(function ()

                    {
                    if (!common.KeepLockOnPolicy(&quot; , &quot;'&quot; , &quot;Zk9fEY7A7HjdhRRYRZFtidpzgT6UwZCEJWNFrpN7&quot; , &quot;'&quot; , &quot;))
                        clearInterval(keeplLockOnPolicyInterval);
                }, 60000);
            });

            


    

        
        
        
        
    






function OptanonWrapper(){window.dataLayer.push({event:&quot;OneTrustGroupsUpdated&quot;})};

(function(){var a=document.createElement(&quot;script&quot;);a.id=&quot;ws-diddyn-lib&quot;;a.type=&quot;text/javascript&quot;;a.async=!0;a.defer=!0;a.src=&quot;//cdn.wannaspeak.com/lib/did.js&quot;;var b=document.getElementsByTagName(&quot;script&quot;)[0];b.parentNode.insertBefore(a,b)})();
(function(c,d,f,g,e){c[e]=c[e]||[];var h=function(){var b={ti:&quot; 4010679&quot;};b.q=c[e];c[e]=new UET(b);c[e].push(&quot;pageLoad&quot;)};var a=d.createElement(f);a.src=g;a.async=1;a.onload=a.onreadystatechange=function(){var b=this.readyState;b&amp;&amp;&quot;loaded&quot;!==b&amp;&amp;&quot;complete&quot;!==b||(h(),a.onload=a.onreadystatechange=null)};d=d.getElementsByTagName(f)[0];d.parentNode.insertBefore(a,d)})(window,document,&quot;script&quot;,&quot;//bat.bing.com/bat.js&quot;,&quot;uetq&quot;);

  
  

var axel=Math.random()+&quot;&quot;,a=1E13*axel;document.write(&quot;\x3ciframe src\x3dhttps://6415832.fls.doubleclick.net/activityi;src\x3d6415832;type\x3dhpt5r0;cat\x3dhisco0;dc_lat\x3d;dc_rdid\x3d;tag_for_child_directed_treatment\x3d;tfua\x3d;npa\x3d;gdpr\x3d${GDPR};gdpr_consent\x3d${GDPR_CONSENT_755};ord\x3d1;num\x3d&quot;+a+&quot; , &quot;'&quot; , &quot;? width\x3d&quot;1&quot; height\x3d&quot;1&quot; frameborder\x3d&quot;0&quot; style\x3d&quot;display:none&quot;\x3e\x3c/iframe\x3e&quot; , &quot;'&quot; , &quot;);


window._fs_host=&quot;eu1.fullstory.com&quot;;window._fs_script=&quot;edge.eu1.fullstory.com/s/fs.js&quot;;window._fs_org=&quot;o-18D5-eu1&quot;;window._fs_namespace=&quot;FS&quot;;
(function(e,k,l,m,h,f,a,d){l in e?e.console&amp;&amp;e.console.log&amp;&amp;e.console.log(&quot; , &quot;'&quot; , &quot;FullStory namespace conflict. Please set window[&quot;_fs_namespace&quot;].&quot; , &quot;'&quot; , &quot;):(a=e[l]=function(c,b,g){a.q?a.q.push([c,b,g]):a._api(c,b,g)},a.q=[],f=k.createElement(m),f.async=1,f.crossOrigin=&quot;anonymous&quot;,f.src=&quot;https://&quot;+_fs_script,d=k.getElementsByTagName(m)[0],d.parentNode.insertBefore(f,d),a.identify=function(c,b,g){a(h,{uid:c},g);b&amp;&amp;a(h,b,g)},a.setUserVars=function(c,b){a(h,c,b)},a.event=function(c,b,g){a(&quot;event&quot;,{n:c,p:b},g)},a.anonymize=
function(){a.identify(!1)},a.shutdown=function(){a(&quot;rec&quot;,!1)},a.restart=function(){a(&quot;rec&quot;,!0)},a.log=function(c,b){a(&quot;log&quot;,[c,b])},a.consent=function(c){a(&quot;consent&quot;,!arguments.length||c)},a.identifyAccount=function(c,b){f=&quot;account&quot;;b=b||{};b.acctId=c;a(f,b)},a.clearUserCookie=function(){},a.setVars=function(c,b){a(&quot;setVars&quot;,[c,b])},a._w={},d=&quot;XMLHttpRequest&quot;,a._w[d]=e[d],d=&quot;fetch&quot;,a._w[d]=e[d],e[d]&amp;&amp;(e[d]=function(){return a._w[d].apply(this,arguments)}),a._v=&quot;1.3.0&quot;)})(window,document,window._fs_namespace,
&quot;script&quot;,&quot;user&quot;);




  window.teads_e=window.teads_e||[];window.teads_adv_id=19472;
Centre de préférences de la confidentialitéLorsque vous consultez un site Web, des données peuvent être stockées dans votre navigateur ou récupérées à partir de celui-ci, généralement sous la forme de cookies. Ces informations peuvent porter sur vous, sur vos préférences ou sur votre appareil et sont principalement utilisées pour s&quot; , &quot;'&quot; , &quot;assurer que le site Web fonctionne correctement. Les informations ne permettent généralement pas de vous identifier directement, mais peuvent vous permettre de bénéficier d&quot; , &quot;'&quot; , &quot;une expérience Web personnalisée. Parce que nous respectons votre droit à la vie privée, nous vous donnons la possibilité de ne pas autoriser certains types de cookies. Cliquez sur les différentes catégories pour obtenir plus de détails sur chacune d&quot; , &quot;'&quot; , &quot;entre elles, et modifier les paramètres par défaut. Toutefois, si vous bloquez certains types de cookies, votre expérience de navigation et les services que nous sommes en mesure de vous offrir peuvent être impactés. Pour modifier vos choix et préférences à tout moment, rendez-vous sur notre lien ‘politique de cookies’ :  
            https://www.hiscox.fr/politique-de-cookiesTout autoriserGérer les préférences de consentementCookies strictement nécessairesToujours actifIl s’agit des cookies indispensables au fonctionnement d’un site web. Sont inclus, par exemple, les cookies qui permettent aux utilisateurs de se connecter à des espaces sécurisés du site web, d’utiliser un panier d’achat ou d’utiliser les services de facturation électronique. Cliquez ici pour une liste d’exemples de cookies strictement nécessaires.Cookies d’analyse et de performance  Cookies d’analyse et de performance Ces cookies nous permettent de déterminer le nombre de visites et les sources du trafic sur notre site web, afin d&quot; , &quot;'&quot; , &quot;en mesurer et d’en améliorer les performances. Ils nous aident également à identifier les pages les plus / moins visitées et à évaluer comment les visiteurs naviguent sur le site. Toutes les informations, collectées par ces cookies, sont agrégées et donc anonymisées. Si vous n&quot; , &quot;'&quot; , &quot;acceptez pas cette catégorie de cookies, nous ne pourrons pas savoir quand vous avez réalisé votre visite sur notre site web.Cookies de fonctionalité  Cookies de fonctionalité Ils sont utilisés pour reconnaître les utilisateurs lorsqu&quot; , &quot;'&quot; , &quot;ils reviennent sur un site web. Ils permettent la personnalisation du contenu, la reconnaissance des utilisateurs et mémorisent les préférences des utilisateurs (par exemple, leur choix de langue ou de région). Cliquez ici pour une liste d’exemples de cookies de fonctionnalités.Cookies pour une publicité ciblée  Cookies pour une publicité ciblée Ces cookies peuvent être activés sur notre site web par nos partenaires publicitaires. Ils peuvent être utilisés par ces entreprises pour établir des profils sur vos intérêts, et afin de vous proposer des publicités ciblées sur d’autres sites. Ils fonctionnement uniquement en identifiant votre navigateur et votre appareil. Si vous n&quot; , &quot;'&quot; , &quot;acceptez pas cette catégorie de cookies, des publicités moins ciblées sur vos intérêts vous seront proposées lors de votre navigation sur d&quot; , &quot;'&quot; , &quot;autres sites web.Cookies de tiers  Cookies de tiers Lorsque vous visitez notre site web, vous pouvez remarquer des cookies qui ne nous sont pas liés. Lorsque vous consultez une page proposant du contenu intégré, par exemple sur YouTube, des cookies peuvent vous être envoyés à partir de ces sites web. Nous ne contrôlons pas la configuration de ces cookies et nous vous conseillons de consulter les sites web des tiers pour plus d&quot; , &quot;'&quot; , &quot;informations sur ces cookies et sur la façon de les gérer.

Pour désactiver toutes les publicités Hiscox ciblées sur les sites web d&quot; , &quot;'&quot; , &quot;autres sociétés, veuillez consulter http://www.youronlinechoices.com/fr/ , où vous pourrez désactiver les réseaux de publicité Microsoft, Amazon, Google, AppNexus, Turn, Facebook et Twitter. Veuillez noter qu&quot; , &quot;'&quot; , &quot;en désactivant les réseaux de publicité listés, vous désactiverez les publicités ciblées pour Hiscox et toute autre entreprise utilisant ces réseaux.

Hiscox utilise des fournisseurs qui peuvent placer des cookies sur le site web d’Hiscox afin de fournir divers services.Back ButtonListe des cookies Search IconFilter IconClear checkbox label labelApply CancelConsent Leg.Interest checkbox label label checkbox label label checkbox label label Confirmer la sélection(function(){var a=document.createElement(&quot;script&quot;);a.id=&quot;ws-diddyn-lib&quot;;a.type=&quot;text/javascript&quot;;a.async=!0;a.defer=!0;a.src=&quot;//cdn.wannaspeak.com/lib/did.js&quot;;var b=document.getElementsByTagName(&quot;script&quot;)[0];b.parentNode.insertBefore(a,b)})();
  
  

var axel=Math.random()+&quot;&quot;,a=1E13*axel;document.write(&quot;\x3ciframe src\x3dhttps://6415832.fls.doubleclick.net/activityi;src\x3d6415832;type\x3dhpt5r0;cat\x3dhisco0;dc_lat\x3d;dc_rdid\x3d;tag_for_child_directed_treatment\x3d;tfua\x3d;npa\x3d;gdpr\x3d${GDPR};gdpr_consent\x3d${GDPR_CONSENT_755};ord\x3d1;num\x3d&quot;+a+&quot; , &quot;'&quot; , &quot;? width\x3d&quot;1&quot; height\x3d&quot;1&quot; frameborder\x3d&quot;0&quot; style\x3d&quot;display:none&quot;\x3e\x3c/iframe\x3e&quot; , &quot;'&quot; , &quot;);


window._fs_host=&quot;eu1.fullstory.com&quot;;window._fs_script=&quot;edge.eu1.fullstory.com/s/fs.js&quot;;window._fs_org=&quot;o-18D5-eu1&quot;;window._fs_namespace=&quot;FS&quot;;
(function(e,k,l,m,h,f,a,d){l in e?e.console&amp;&amp;e.console.log&amp;&amp;e.console.log(&quot; , &quot;'&quot; , &quot;FullStory namespace conflict. Please set window[&quot;_fs_namespace&quot;].&quot; , &quot;'&quot; , &quot;):(a=e[l]=function(c,b,g){a.q?a.q.push([c,b,g]):a._api(c,b,g)},a.q=[],f=k.createElement(m),f.async=1,f.crossOrigin=&quot;anonymous&quot;,f.src=&quot;https://&quot;+_fs_script,d=k.getElementsByTagName(m)[0],d.parentNode.insertBefore(f,d),a.identify=function(c,b,g){a(h,{uid:c},g);b&amp;&amp;a(h,b,g)},a.setUserVars=function(c,b){a(h,c,b)},a.event=function(c,b,g){a(&quot;event&quot;,{n:c,p:b},g)},a.anonymize=
function(){a.identify(!1)},a.shutdown=function(){a(&quot;rec&quot;,!1)},a.restart=function(){a(&quot;rec&quot;,!0)},a.log=function(c,b){a(&quot;log&quot;,[c,b])},a.consent=function(c){a(&quot;consent&quot;,!arguments.length||c)},a.identifyAccount=function(c,b){f=&quot;account&quot;;b=b||{};b.acctId=c;a(f,b)},a.clearUserCookie=function(){},a.setVars=function(c,b){a(&quot;setVars&quot;,[c,b])},a._w={},d=&quot;XMLHttpRequest&quot;,a._w[d]=e[d],d=&quot;fetch&quot;,a._w[d]=e[d],e[d]&amp;&amp;(e[d]=function(){return a._w[d].apply(this,arguments)}),a._v=&quot;1.3.0&quot;)})(window,document,window._fs_namespace,
&quot;script&quot;,&quot;user&quot;);
  window.teads_e=window.teads_e||[];window.teads_adv_id=19472;
id(&quot;LastName--mdc&quot;)&quot;))]</value>
      <webElementGuid>799a01ad-c325-4bd4-8f1d-40890fd3a075</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
